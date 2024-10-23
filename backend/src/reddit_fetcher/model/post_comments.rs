use crate::cast;
use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::model::reddit_data::RedditData;
use crate::reddit_fetcher::nlp_request::{DataSource, RMoodsNlpRequest};
use crate::reddit_fetcher::reddit::model::{MoreComments, RawComment, RawContainer};
use crate::reddit_fetcher::reddit::request::PostCommentsRequest;
use log::debug;
use log_derive::logfn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostComments {
    pub list: Vec<RawComment>,
    pub more: Vec<MoreComments>,
}

impl RedditData for PostComments {
    type RequestType = PostCommentsRequest;

    #[logfn(err = "ERROR", fmt = "Failed to parse from RedditContainer: {0}")]
    fn from_reddit_container(container: RawContainer) -> Result<PostComments, FetcherError> {
        let mut comments: Vec<RawComment> = Vec::new();

        let listing = cast!(container, RawContainer::Listing)?;
        let mut mores = vec![];

        for child in listing.children {
            match child {
                RawContainer::Comment(mut comment) => {
                    let (mut replies, mut mores_2) = flattened_replies(&comment)?;
                    mores.append(&mut mores_2);
                    comments.append(&mut replies);

                    comment.replies = None;
                    comments.push(*comment);
                }
                RawContainer::More(more) => mores.push(*more),
                _ => {
                    return Err(FetcherError::RedditParseError(
                        "Failed to parse comment from Reddit container".to_string(),
                    )
                    .into());
                }
            }
        }

        debug!("Returning {} post replies", { comments.len() });

        Ok(PostComments {
            list: comments,
            more: mores,
        })
    }

    fn create_reddit_request(
        request: &RMoodsNlpRequest,
        source: DataSource,
        after: Option<String>,
    ) -> Self::RequestType {
        PostCommentsRequest {
            subreddit: source.name,
            post_id: source.post_id.expect("Must be here"),
            sorting: request.sorting,
            after,
        }
    }

    fn concat(&mut self, other: Self) -> Self {
        Self {
            list: [self.list.clone(), other.list].concat(),
            more: [self.more.clone(), other.more].concat(),
        }
    }
}

fn flatten_replies_internal(
    comment: &RawComment,
    all_replies: &mut Vec<RawComment>,
    mores: &mut Vec<MoreComments>,
    depth: u16,
) -> Result<(), FetcherError> {
    if let Some(replies_container) = comment.replies() {
        let listing = cast!(replies_container, RawContainer::Listing)?;

        let mut replies = vec![];
        for raw_reply in listing.children.clone() {
            // replies.push(cast!(raw_reply, RawContainer::Comment)?);
            match raw_reply {
                RawContainer::Comment(reply) => replies.push(*reply),
                RawContainer::More(more) => mores.push(*more),
                _ => {
                    return Err(FetcherError::RedditParseError(
                        "Failed to parse comment from Reddit container".to_string(),
                    )
                    .into());
                }
            }
        }

        let tabs = "  ".repeat((depth + 1).into());

        for mut reply in replies {
            debug!("{tabs}u/{}", reply.author());
            let _ = flatten_replies_internal(&reply, all_replies, mores, depth + 1)?;
            reply.replies = None;
            all_replies.push(reply.clone());
        }
    }
    Ok(())
}

#[logfn(err = "ERROR", fmt = "Failed to flatten replies: {0}")]
fn flattened_replies(
    comment: &RawComment,
) -> Result<(Vec<RawComment>, Vec<MoreComments>), FetcherError> {
    let mut all_replies = vec![];
    let mut mores = vec![];

    let _ = flatten_replies_internal(comment, &mut all_replies, &mut mores, 0)?;

    Ok((all_replies, mores))
}
