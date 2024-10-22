use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::model::reddit_data::RedditData;
use crate::reddit_fetcher::nlp_request::RMoodsNlpRequest;
use crate::reddit_fetcher::reddit::{
    connection::RedditConnection,
    error::RedditError,
    model::{MoreComments, RawComment},
};
use log::{debug, info};
use log_derive::logfn;

#[derive(Clone)]
pub struct RMoodsFetcher {
    reddit_connection: RedditConnection,
}

impl RMoodsFetcher {
    #[logfn(err = "ERROR", fmt = "Failed to create Reddit fetcher: {0}")]
    pub async fn new(http: reqwest::Client) -> Result<Self, RedditError> {
        let reddit_connection = RedditConnection::new(http).await?;
        Ok(Self { reddit_connection })
    }

    #[logfn(err = "ERROR", fmt = "Failed to fetch feed: {0}")]
    pub async fn fetch_feed<T: RedditData>(
        &mut self,
        request: RMoodsNlpRequest,
    ) -> Result<(T, u16), FetcherError> {
        info!("Fetching feed: {:?}", request);

        let requests_to_make = request.size.clone().into();
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
}
