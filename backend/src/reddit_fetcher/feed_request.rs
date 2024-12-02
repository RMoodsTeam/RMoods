use crate::nlp::analysis::NlpAnalysisKind;
use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::reddit::request::feed_sorting::FeedSorting;
use axum::async_trait;
use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use http::StatusCode;
use log_derive::logfn;
use serde::Deserialize;
use std::fmt::Debug;

/// What kind of feed do we fetch and make a report on?
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RedditFeedKind {
    UserPosts,
    PostComments,
    SubredditPosts,
}

/// Represents a data source for the Reddit API.
/// It can be a user, a subreddit, or a post.
///
/// * The `post_id` field is only used when fetching comments for a post.
/// * The `share` field is used to calculate the share of the report that this data source represents.
///   * It should be a number between 0 and 1.
///   * The sum of all shares should be 1.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataSource {
    pub name: String,
    pub post_id: Option<String>, // Only for PostComments
    pub share: u8,
}

/// Represents a request to fetch a feed from Reddit.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetcherFeedRequest {
    /// Determines what kind of feed do we fetch and make a report on.
    pub resource_kind: RedditFeedKind,
    /// Determines what NLP reports do we want to generate.
    pub analyses: Vec<NlpAnalysisKind>,
    /// Determines the data sources for the feed.
    pub data_sources: Vec<DataSource>,
    /// Determines how many posts do we want to use to fulfill that report request.
    pub size: u16,
    /// Determines the sorting of the feed.
    pub sorting: FeedSorting,
}

impl FetcherFeedRequest {
    /// Validate a feed request.
    ///
    /// * Data sources cannot be empty.
    /// * The sum of all shares should be 100.
    /// * Data sources should have a share greater than 0.
    /// * Data source names should only contain alphanumeric characters.
    /// * Post IDs should only contain alphanumeric characters.
    /// * All data sources for PostComments should have a `post_id`.
    /// * No data sources for UserPosts and SubredditPosts should have a `post_id`.
    #[logfn(err = "ERROR", fmt = "Failed to validate feed request: {0}")]
    pub fn validate(&self) -> Result<(), FetcherError> {
        // Check if there are any data sources
        if self.data_sources.is_empty() {
            return Err(FetcherError::InvalidFeedRequest(
                "Data sources cannot be empty".to_string(),
            ));
        }

        // Check if the sum of all shares is 100
        let sum: u8 = self.data_sources.iter().map(|ds| ds.share).sum();
        if sum != 100 {
            return Err(FetcherError::InvalidFeedRequest(
                "The sum of all shares should be 100".to_string(),
            ));
        }

        // Disallow any data sources with share = 0
        if self.data_sources.iter().any(|ds| ds.share == 0) {
            return Err(FetcherError::InvalidFeedRequest(
                "Data sources should have a share greater than 0".to_string(),
            ));
        }

        // Check if data source names contain illegal characters
        let char_is_legal = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == '-';
        let names_contain_illegal_chars = self
            .data_sources
            .iter()
            .any(|ds| ds.name.chars().any(|c| !char_is_legal(c)));
        if names_contain_illegal_chars {
            return Err(FetcherError::InvalidFeedRequest(
                "Data source names should only contain alphanumeric characters".to_string(),
            ));
        }

        // Check if post_id contains illegal characters
        let post_ids_contain_illegal_chars = self.data_sources.iter().any(|ds| {
            ds.post_id.is_some()
                && ds
                    .post_id
                    .as_ref()
                    .unwrap()
                    .chars()
                    .any(|c| !c.is_ascii_alphanumeric())
        });
        if post_ids_contain_illegal_chars {
            return Err(FetcherError::InvalidFeedRequest(
                "Post IDs should only contain alphanumeric characters".to_string(),
            ));
        }

        // Check if data sources are declared correctly
        if self.resource_kind == RedditFeedKind::PostComments {
            if self.data_sources.iter().any(|ds| ds.post_id.is_none()) {
                return Err(FetcherError::InvalidFeedRequest(
                    "All data sources for PostComments should have a post_id".to_string(),
                ));
            }
        } else {
            if self.data_sources.iter().any(|ds| ds.post_id.is_some()) {
                return Err(FetcherError::InvalidFeedRequest(
                    "No data sources for UserPosts and SubredditPosts should have a post_id"
                        .to_string(),
                ));
            }
        }

        Ok(())
    }
}

#[async_trait]
impl<S> FromRequest<S> for FetcherFeedRequest
where
    S: Send + Sync,
{
    type Rejection = StatusCode;
    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        log::debug!("Extracting FetcherFeedRequest");
        let body = match Bytes::from_request(request, state).await {
            Ok(body) => body,
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        };
        match serde_json::from_slice(&body) {
            Ok(feed_request) => {
                log::debug!("Extracted FetcherFeedRequest successfully");
                Ok(feed_request)
            }
            Err(e) => {
                log::error!("Failed to extract FetcherFeedRequest: {e:?}");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    const JSON: &str = r#"
        {
            "resourceKind": "userPosts",
            "analyses": ["language", "sentiment"],
            "dataSources": [
                {
                    "name": "username",
                    "share": 50
                },
                {
                    "name": "username2",
                    "share": 50
                }
            ],
            "size": 10,
            "sorting": {
              "kind": "hot"
            }
        }
    "#;

    #[test]
    fn test_deserialize_feed_request() {
        let feed_request: super::FetcherFeedRequest = serde_json::from_str(JSON).unwrap();
        assert_eq!(feed_request.resource_kind, super::RedditFeedKind::UserPosts);
        assert_eq!(
            feed_request.analyses,
            vec![
                super::NlpAnalysisKind::Language,
                super::NlpAnalysisKind::Sentiment
            ]
        );
        assert_eq!(feed_request.data_sources.len(), 2);
        assert_eq!(feed_request.size, 10);
        assert_eq!(
            feed_request.sorting,
            crate::reddit_fetcher::reddit::request::feed_sorting::FeedSorting::Hot
        );
    }

    fn testing_request() -> super::FetcherFeedRequest {
        serde_json::from_str(JSON).unwrap()
    }

    #[test]
    fn test_validate_feed_request_with_valid_data() {
        let feed_request = testing_request();
        assert!(feed_request.validate().is_ok());
    }

    #[test]
    fn test_validate_feed_request_empty_data_sources() {
        let mut feed_request = testing_request();
        feed_request.data_sources.clear();
        assert!(feed_request.validate().is_err());
    }

    #[test]
    fn test_validate_feed_request_shares_not_sum_to_100() {
        let mut feed_request = testing_request();
        feed_request.data_sources = vec![
            super::DataSource {
                name: "username".to_string(),
                post_id: None,
                share: 50,
            },
            super::DataSource {
                name: "username2".to_string(),
                post_id: None,
                share: 40,
            },
        ];
        assert!(feed_request.validate().is_err());
    }

    #[test]
    fn test_validate_feed_request_zero_share() {
        let mut feed_request = testing_request();
        feed_request.data_sources = vec![
            super::DataSource {
                name: "username".to_string(),
                post_id: None,
                share: 50,
            },
            super::DataSource {
                name: "username2".to_string(),
                post_id: None,
                share: 0,
            },
        ];
        assert!(feed_request.validate().is_err());
    }

    #[test]
    fn test_validate_feed_request_illegal_characters_in_names() {
        let mut feed_request = testing_request();
        feed_request.data_sources = vec![
            super::DataSource {
                name: "username".to_string(),
                post_id: None,
                share: 50,
            },
            super::DataSource {
                name: "username2!".to_string(),
                post_id: None,
                share: 50,
            },
        ];
        assert!(feed_request.validate().is_err());
    }

    #[test]
    fn test_validate_feed_request_illegal_characters_in_post_id() {
        let mut feed_request = testing_request();
        feed_request.data_sources = vec![
            super::DataSource {
                name: "username".to_string(),
                post_id: Some("post_id".to_string()),
                share: 50,
            },
            super::DataSource {
                name: "username2".to_string(),
                post_id: Some("post_id!".to_string()),
                share: 50,
            },
        ];
        assert!(feed_request.validate().is_err());
    }

    #[test]
    fn test_validate_feed_request_incorrectly_declared_data_sources() {
        let mut feed_request = testing_request();
        feed_request.resource_kind = super::RedditFeedKind::PostComments;
        feed_request.data_sources = vec![
            super::DataSource {
                name: "username".to_string(),
                post_id: None,
                share: 50,
            },
            super::DataSource {
                name: "username2".to_string(),
                post_id: Some("post_id".to_string()),
                share: 50,
            },
        ];
        assert!(feed_request.validate().is_err());
    }
}
