use crate::reddit_fetcher::model::post_comments::PostComments;
use crate::reddit_fetcher::model::posts::Posts;
use crate::reddit_fetcher::model::user_posts::UserPosts;
use dyn_clone::DynClone;
use std::fmt::Debug;

#[typetag::serialize(tag = "type")]
pub trait RMoodsReport: Debug + Send + DynClone {}

/// Temporary solution until we add proper report result structs.
/// TODO: Add proper report result structs.
#[typetag::serialize]
impl RMoodsReport for Posts {}

#[typetag::serialize]
impl RMoodsReport for PostComments {}

#[typetag::serialize]
impl RMoodsReport for UserPosts {}

dyn_clone::clone_trait_object!(RMoodsReport);
