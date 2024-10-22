use crate::reddit_fetcher::reddit::request::params::FeedSorting;

/// What kind of data do we fetch and make a report on?
#[derive(Debug)]
pub enum RedditFeedKind {
    UserPosts,
    PostComments,
    SubredditPosts,
}

// TODO: Add more types
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
