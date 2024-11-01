use crate::reddit_fetcher::feed_request::FetcherFeedRequest;
use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::model::reddit_data::{RedditAboutData, RedditFeedData};
use crate::reddit_fetcher::reddit::{
    connection::RedditConnection,
    error::RedditError,
    model::{MoreComments, RawComment},
};
use log::{debug, info};
use log_derive::logfn;

/// Layer responsible for fetching data from Reddit.
/// * It uses a `RedditConnection` to make requests to the Reddit API.
/// * It's responsible for handling pagination and fetching more comments.
/// * It's also responsible for parsing the raw data into the desired format.
/// * It's a thin layer over the `RedditConnection` and the data models of the low-level Reddit module.
#[derive(Clone)]
pub struct RMoodsFetcher {
    reddit_connection: RedditConnection,
}

impl RMoodsFetcher {
    /// Create a new instance of RMoodsFetcher with the provided `http` client.
    /// There should exist only one instance of this struct in the application.
    #[logfn(err = "ERROR", fmt = "Failed to create Reddit fetcher: {0}")]
    pub async fn new(http: reqwest::Client) -> Result<Self, RedditError> {
        let reddit_connection = RedditConnection::new(http).await?;
        Ok(Self { reddit_connection })
    }

    /// Fetches a feed of Reddit data.
    /// * It fetches the data from the Reddit API using the provided `FetcherFeedRequest`.
    /// * It fetches the data in multiple requests if needed.
    /// * It returns the parsed data and the number of requests made.
    /// * The parsed data is of type `T` which should implement the `RedditFeedData` trait.
    #[logfn(err = "ERROR", fmt = "Failed to fetch feed: {0}")]
    pub async fn fetch_feed<T: RedditFeedData>(
        &mut self,
        request: FetcherFeedRequest,
    ) -> Result<(T, u16), FetcherError> {
        info!("Fetching feed: {:?}", request);

        let requests_to_make = request.size.clone().into();
        // TODO: allow for multiple data sources
        let source = request.data_sources.first().unwrap().clone();

        let initial_request = T::create_reddit_request(&request, source.clone(), None);
        let (raw_data, mut after) = self.reddit_connection.fetch_raw(initial_request).await?;
        let mut parsed = T::from_reddit_container(raw_data)?;

        let mut requests_made = 1;
        while requests_made < requests_to_make && after.is_some() {
            let next_request = T::create_reddit_request(&request, source.clone(), after.clone());
            let (raw_data, next_after) = self.reddit_connection.fetch_raw(next_request).await?;
            parsed = T::from_reddit_container(raw_data)?.concat(parsed);

            after = next_after;
            requests_made += 1;
        }

        debug!("Requests made: {}/{}", requests_made, requests_to_make);
        info!("Done fetching feed: {:?}", request);

        Ok((parsed, requests_made))
    }

    /// Uses the MoreComments stubs to fetch more comments.
    /// * It fetches the comments from the Reddit API using the provided `MoreComments` stubs.
    /// * It fetches the comments in multiple requests if needed.
    /// * It returns the parsed comments.
    /// The resulting list of comments is to be appended to the original PostComments struct.
    /// To obtain the MoreComments stubs, first fetch a feed of comments and extract the `more` field.
    #[logfn(err = "ERROR", fmt = "Fetcher - Failed to fetch more comments: {0}")]
    pub async fn fetch_more_comments(
        &mut self,
        stubs: &[MoreComments],
        requests_left: u16,
    ) -> Result<Vec<RawComment>, FetcherError> {
        let mut requests_left = requests_left;
        let mut comments = vec![];

        debug!(
            "Fetching more comments with {} requests left",
            requests_left
        );

        for more_comments in stubs.into_iter() {
            if requests_left == 0 {
                break;
            }
            let (new_comments, requests_made) = self
                .reddit_connection
                .fetch_more_comments(more_comments, requests_left)
                .await?;

            requests_left -= requests_made;
            info!("Requests left: {}", requests_left);
            comments.extend(new_comments);
        }

        Ok(comments)
    }

    /// Fetches simple data from the Reddit API.
    /// "About" data is data about a subreddit or a user, obtained through the user/about.json and subreddit/about.json paths.
    /// This is a very simple operation, as the data is not paginated or parsed in any special way.
    #[logfn(err = "ERROR", fmt = "Failed to fetch about: {0}")]
    pub async fn fetch_about<T: RedditAboutData>(
        &mut self,
        request: T::RequestType,
    ) -> Result<T, FetcherError> {
        let raw = self.reddit_connection.fetch_raw(request).await?;
        log::info!("Parsing...");
        let data = T::from_reddit_container(raw.0)?;
        Ok(data)
    }
}
