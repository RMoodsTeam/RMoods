use crate::reddit::{
    connection::RedditConnection,
    request::{
        params::FeedSorting, PostCommentsRequest, RedditResource, SubredditPostsRequest,
        UserPostsRequest,
    },
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
    pub postId: Option<String>, // Only for PostComments
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
    resource_kind: RedditFeedKind,
    report_types: Vec<RMoodsReportType>,
    data_sources: Vec<DataSource>,
    size: RequestSize,
    sorting: FeedSorting,
}

pub struct RMoodsFetcher {
    reddit_connection: RedditConnection,
}

fn convert(request: &RMoodsNlpRequest, source: DataSource) -> Box<dyn RedditResource> {
    match request.resource_kind {
        RedditFeedKind::UserPosts => Box::new(UserPostsRequest {
            username: source.name,
            sorting: request.sorting,
        }),
        RedditFeedKind::PostComments => Box::new(PostCommentsRequest {
            subreddit: source.name,
            post_id: source.postId.unwrap(),
            sorting: request.sorting,
        }),
        RedditFeedKind::SubredditPosts => Box::new(SubredditPostsRequest {
            subreddit: source.name,
            sorting: request.sorting,
        }),
    }
}

impl RMoodsFetcher {
    pub fn new(reddit_connection: RedditConnection) -> Self {
        Self { reddit_connection }
    }

    pub async fn fetch_feed(&mut self, request: RMoodsNlpRequest) -> anyhow::Result<()> {
        dbg!(&request);

        let request_number = u16::from(request.size.clone());

        let source = request.data_sources.first().unwrap();

        dbg!(&source);
        let reddit_request = convert(&request, source.clone());
        let raw_data = self.reddit_connection.fetch_raw(reddit_request).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rmoods_fetcher() {
        let request = RMoodsNlpRequest {
            resource_kind: RedditFeedKind::UserPosts,
            report_types: vec![RMoodsReportType::Sentiment],
            data_sources: vec![DataSource {
                name: "u/utoipa".to_string(),
                postId: None,
                share: 1.0,
            }],
            size: RequestSize::Medium,
            sorting: Default::default(),
        };

        let result = RMoodsFetcher::fetch_feed(request);

        assert!(result.is_ok());
    }
}
