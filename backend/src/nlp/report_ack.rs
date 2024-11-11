use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub struct ReportAck {
    status: StatusCode,
    message: String,
}

impl ReportAck {
    pub fn new() -> Self {
        ReportAck {
            status: StatusCode::ACCEPTED,
            message: "Report request received. Processing".to_string(),
        }
    }
}

impl IntoResponse for ReportAck {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(json!({
                "status": self.status.to_string(),
                "message": self.message,
            })),
        )
            .into_response()
    }
}
