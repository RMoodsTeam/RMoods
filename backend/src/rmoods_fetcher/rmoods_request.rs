use log::{debug, info, warn};

use crate::{
    reddit::{
        connection::RedditConnection,
        error::RedditError,
        model::{MoreComments, RawComment},
        request::{params::FeedSorting, RedditResource},
    },
    rmoods_fetcher::RedditData,
};

/// What kind of data do we fetch and make a report on?
#[derive(Debug)]
pub enum RedditFeedKind {
    UserPosts,
    PostComments,
    SubredditPosts,
}

/// What NLP reports do we want to generate?
#[derive(Debug)]
pub enum RMoodsReportType {
    Sentiment,
    Sarcasm,
    Etc,
}

/// Represents a data source for the Reddit API.
#[derive(Debug, Clone)]
pub struct DataSource {
    pub name: String,
    pub post_id: Option<String>, // Only for PostComments
    pub share: f32,
}

#[derive(Default, Debug, Clone)]
pub enum RequestSize {
    Small,
    #[default]
    Medium,
    Large,
    Custom(u16),
}

impl From<RequestSize> for u16 {
    fn from(value: RequestSize) -> Self {
        match value {
            RequestSize::Small => 50,
            RequestSize::Medium => 250,
            RequestSize::Large => 500,
            RequestSize::Custom(n) => n,
        }
    }
}

#[derive(Debug)]
pub struct RMoodsNlpRequest {
    pub resource_kind: RedditFeedKind,
    pub report_types: Vec<RMoodsReportType>,
    pub data_sources: Vec<DataSource>,
    pub size: RequestSize,
    pub sorting: FeedSorting,
}

#[derive(Clone)]
pub struct RMoodsFetcher {
    reddit_connection: RedditConnection,
}

impl RMoodsFetcher {
    pub async fn new(http: reqwest::Client) -> Result<Self, RedditError> {
        let reddit_connection = RedditConnection::new(http).await?;
        Ok(Self { reddit_connection })
    }

    pub async fn fetch_feed<T: RedditData>(
        &mut self,
        request: RMoodsNlpRequest,
    ) -> anyhow::Result<(T, u16)> {
        dbg!(&request);
        info!("Fetching feed: {:?}", request);

        let requests_to_make = u16::from(request.size.clone());

        let source = request.data_sources.first().unwrap().clone();

        // Initial request, no `after` parameter
        let initial_request = T::create_reddit_request(&request, source.clone(), None);
        dbg!("{:?}", initial_request.into_request_parts());
        let (raw_data, mut after) = self
            .reddit_connection
            .fetch_raw(initial_request)
            .await
            .unwrap();
        let mut parsed = T::from_reddit_container(raw_data).unwrap();
        let mut requests_made = 1;

        // Loop until `after` becomes `None` or we reach the limit of requests
        while requests_made < requests_to_make && after.is_some() {
            let next_request = T::create_reddit_request(&request, source.clone(), after.clone());
            let (raw_data, next_after) = self
                .reddit_connection
                .fetch_raw(next_request)
                .await
                .unwrap();
            parsed = T::from_reddit_container(raw_data).unwrap().concat(parsed); // Chain parsed data
            after = next_after; // Update `after`
            requests_made += 1;
        }

        debug!("Requests made: {}/{}", requests_made, requests_to_make);
        debug!("After: {:?}", after);

        info!("Done fetching feed: {:?}", request);

        Ok((parsed, requests_made))
    }

    pub async fn fetch_more_comments(
        &mut self,
        stubs: &[MoreComments],
        requests_left: u16,
    ) -> anyhow::Result<Vec<RawComment>> {
        let mut requests_left = requests_left;
        let mut comments = vec![];

        debug!("Fetching more comments with {} requests left", requests_left);

        for more_comments in stubs {
            if requests_left == 0 {
                break;
            }
            let (new_comments, requests_made) = self
                .reddit_connection
                .fetch_more_comments(&more_comments, requests_left)
                .await
                .unwrap();

            requests_left -= requests_made;
            info!("Requests left: {}", requests_left);
            comments.extend(new_comments);
        }

        Ok(comments)
    }
}
