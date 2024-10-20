use crate::{
    reddit::{
        connection::RedditConnection,
        model::{MoreComments, RawComment},
        request::{
            params::FeedSorting, PostCommentsRequest, RedditResource, SubredditPostsRequest,
            UserPostsRequest,
        },
    },
    rmoods_fetcher::{Posts, RedditData},
};

use super::PostComments;

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

pub struct RMoodsFetcher {
    reddit_connection: RedditConnection,
}

impl RMoodsFetcher {
    pub fn new(reddit_connection: RedditConnection) -> Self {
        Self { reddit_connection }
    }

    pub async fn fetch_feed<T: RedditData>(
        &mut self,
        request: RMoodsNlpRequest,
    ) -> anyhow::Result<(T, u16)> {
        dbg!(&request);

        let requests_to_make = u16::from(request.size.clone());

        let source = request.data_sources.first().unwrap().clone();

        // Initial request, no `after` parameter
        let initial_request = T::create_reddit_request(&request, source.clone(), None);
        let (raw_data, mut after) = self.reddit_connection.fetch_raw(initial_request).await?;
        let mut parsed = T::from_reddit_container(raw_data)?;
        let mut requests_made = 1;

        // Loop until `after` becomes `None`
        while requests_made < requests_to_make && after.is_some() {
            let next_request = T::create_reddit_request(&request, source.clone(), after.clone());
            let (raw_data, next_after) = self.reddit_connection.fetch_raw(next_request).await?;
            parsed = T::from_reddit_container(raw_data)?.concat(parsed); // Chain parsed data
            after = next_after; // Update `after`
            requests_made += 1;
        }

        Ok((parsed, requests_made))
    }

    pub async fn fetch_more_comments(
        post_comments: PostComments,
        requests_left: u16,
    ) -> anyhow::Result<PostComments> {
        todo!()
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
                post_id: None,
                share: 1.0,
            }],
            size: RequestSize::Medium,
            sorting: Default::default(),
        };

        // let result = RMoodsFetcher::fetch_feed(request);

        // assert!(result.is_ok());
    }
}
