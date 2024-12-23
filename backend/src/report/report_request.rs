use crate::fetcher::feed_request::FetcherDataRequest;
use crate::nlp::nlp_request::NlpRequest;
use crate::validation::validated::Validated;
use crate::validation::validation_error::ValidationError;
use axum::async_trait;
use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use http::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportRequest {
    pub title: String,
    pub description: String,
    pub is_public: bool,
    pub data_request: FetcherDataRequest,
    pub nlp_request: NlpRequest,
}

#[async_trait]
impl<S> FromRequest<S> for ReportRequest
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

impl Validated for ReportRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        self.data_request.validate()?;
        self.nlp_request.validate()?;
        Ok(())
    }
}
