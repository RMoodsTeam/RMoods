use crate::fetcher::model::post_comments::PostComments;
use crate::fetcher::model::posts::Posts;
use crate::fetcher::model::user_posts::UserPosts;
use crate::report::reddit_data_container::RedditDataContainer;

pub trait ToTextData {
    fn to_text_data(&self) -> Vec<String>;
}

impl ToTextData for Posts {
    fn to_text_data(&self) -> Vec<String> {
        self.list
            .iter()
            .map(|post| post.selftext.clone()) // Extract post's text
            .filter(|text| !text.is_empty()) // Filter out empty texts
            .collect()
    }
}

impl ToTextData for UserPosts {
    fn to_text_data(&self) -> Vec<String> {
        self.posts
            .iter()
            .map(|post| post.selftext.clone())
            .filter(|text| !text.is_empty())
            .collect()
    }
}

impl ToTextData for PostComments {
    fn to_text_data(&self) -> Vec<String> {
        self.list
            .iter()
            .map(|comment| comment.body.clone()) // Extract comment's text
            .filter(|text| !text.is_empty()) // Filter out empty texts
            .collect()
    }
}

impl ToTextData for RedditDataContainer {
    fn to_text_data(&self) -> Vec<String> {
        match self {
            Self::PostComments(post_comments) => post_comments.to_text_data(),
            Self::SubredditPosts(posts) => posts.to_text_data(),
            Self::UserPosts(user_posts) => user_posts.to_text_data(),
        }
    }
}
