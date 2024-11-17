use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn response_logging_middleware(req: Request, next: Next) -> Response {
    let response = next.run(req).await;
    log::debug!("{:?}", response);
    response
}
