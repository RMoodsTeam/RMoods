use http::StatusCode;

pub async fn spam() -> StatusCode {
    StatusCode::IM_A_TEAPOT
}
