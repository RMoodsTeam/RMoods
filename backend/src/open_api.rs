use utoipa::OpenApi;

use crate::api::*;
use crate::*;

/// OpenAPI documentation for the RMoods server
#[derive(OpenApi)]
#[openapi(paths(
    debug::lorem,
    debug::timeout,
    debug::subreddit_info,
    debug::post_comments,
    debug::user_info,
    debug::subreddit_posts,
    debug::user_posts,
    auth::login::login
))]
pub struct ApiDoc;
