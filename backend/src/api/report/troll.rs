use http::StatusCode;

pub async fn troll() -> StatusCode {
    StatusCode::IM_A_TEAPOT
}
