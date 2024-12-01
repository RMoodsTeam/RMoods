use crate::api::auth::google::GoogleId;
use crate::nlp::report::ReportId;
use sqlx::types::Uuid;

struct DbReport {
    id: Uuid,
    display_id: ReportId,
    user_id: GoogleId,
    is_public: bool,
    metadata_id: Uuid,
    analysis_id: Uuid,
}
