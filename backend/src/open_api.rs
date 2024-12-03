use utoipa::OpenApi;

use crate::api::*;

/// OpenAPI documentation for the RMoods server.
///
/// All routes that should be documented in our interactive docs should be added here.
#[derive(OpenApi)]
#[openapi(paths(auth::login::login))]
pub struct ApiDoc;
