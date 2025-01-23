use crate::fetcher::model::user_posts::PostOrComment;
use crate::fetcher::reddit::model::{RawComment, RawPost};
use std::fmt::Debug;

pub trait RedditFeedItem: Debug {}

impl RedditFeedItem for RawPost {}
impl RedditFeedItem for RawComment {}
impl RedditFeedItem for PostOrComment {}
