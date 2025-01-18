use crate::fetcher::reddit::model::{RawComment, RawPost};

pub trait RedditFeedItem {}

impl RedditFeedItem for RawPost {}
impl RedditFeedItem for RawComment {}
