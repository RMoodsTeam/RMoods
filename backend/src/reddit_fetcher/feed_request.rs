use crate::reddit_fetcher::reddit::request::params::FeedSorting;

/// What kind of feed do we fetch and make a report on?
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
/// It can be a user, a subreddit, or a post.
///
/// * The `post_id` field is only used when fetching comments for a post.
/// * The `share` field is used to calculate the share of the report that this data source represents.
///   * It should be a number between 0 and 1.
///   * The sum of all shares should be 1.
#[derive(Debug, Clone)]
pub struct DataSource {
    pub name: String,
    pub post_id: Option<String>, // Only for PostComments
    pub share: f32,
}

/// How many requests do we want to make to fulfill the report request?
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

/// Represents a request to fetch a feed from Reddit.
#[derive(Debug)]
pub struct FetcherFeedRequest {
    /// Determines what kind of feed do we fetch and make a report on.
    pub resource_kind: RedditFeedKind,
    /// Determines what NLP reports do we want to generate.
    pub report_types: Vec<RMoodsReportType>,
    /// Determines the data sources for the feed.
    pub data_sources: Vec<DataSource>,
    /// Determines how many posts do we want to use to fulfill that report request.
    pub size: RequestSize,
    /// Determines the sorting of the feed.
    pub sorting: FeedSorting,
}
