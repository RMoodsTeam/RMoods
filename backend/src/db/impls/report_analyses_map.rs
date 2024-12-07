use crate::db::db_stored::DbStoredDependentlyInner;
use crate::db::from_db::FromDb;
use crate::db::model::{DbNlpAnalysis, DbReportAnalysesMap};
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::NlpAnalysis;
use crate::nlp::report::ReportAnalysesMap;
use axum::async_trait;
use sqlx::{Error, Postgres, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

#[async_trait]
impl DbStoredDependentlyInner for ReportAnalysesMap {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<Uuid, Error> {
        let mut map = HashMap::new();

        // Process analyses sequentially to avoid problems with escaping references to `tx`
        for (kind, analysis) in &self.analyses {
            let id = analysis.inner_save(tx).await?;
            map.insert(kind.clone(), id);
        }

        type A = NlpAnalysisKind;
        let id = sqlx::query!(
        r#"
        INSERT INTO report_analyses_maps (clickbait_id, hate_speech_id, keywords_id, language_id, politics_id, sarcasm_id, sentiment_id, spam_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id as "id: Uuid";
        "#,
        map.get(&A::Clickbait),
        map.get(&A::HateSpeech),
        map.get(&A::Keywords),
        map.get(&A::Language),
        map.get(&A::Politics),
        map.get(&A::Sarcasm),
        map.get(&A::Sentiment),
        map.get(&A::Spam)
    )
            .fetch_one(&mut **tx)
            .await?
            .id;

        Ok(id)
    }
}

#[async_trait]
impl FromDb for ReportAnalysesMap {
    type DbModel = DbReportAnalysesMap;
    async fn from_db_model(
        model: Self::DbModel,
        tx: &mut Transaction<Postgres>,
    ) -> Result<Self, Error> {
        let analyses = sqlx::query_as!(
            DbNlpAnalysis,
            r#"
            SELECT *
            FROM nlp_analyses
            WHERE nlp_metadata_id IN (
                $1, $2, $3, $4, $5, $6, $7, $8
            )
            "#,
            model.clickbait_id,
            model.hate_speech_id,
            model.keywords_id,
            model.language_id,
            model.politics_id,
            model.sarcasm_id,
            model.sentiment_id,
            model.spam_id
        )
        .fetch_all(&mut **tx)
        .await?;

        let mut analyses_map = HashMap::new();
        for db_analysis in analyses {
            let analysis = NlpAnalysis::from_db_model(db_analysis, tx).await?;
            analyses_map.insert(analysis.kind.clone(), analysis);
        }

        Ok(ReportAnalysesMap {
            analyses: analyses_map,
        })
    }
}
