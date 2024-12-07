use crate::api::auth::google::User;
use crate::db::db_client::DbClient;
use crate::db::model::{
    DbNlpAnalysis, DbNlpMetadata, DbReport, DbReportAnalysesMap, DbReportMetadata,
};
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::{NlpAnalysis, NlpMetadata};
use crate::nlp::report::{Report, ReportAnalysesMap, ReportMetadata};
use axum::async_trait;
use sqlx::{Error, Postgres, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

pub struct DbPagination {
    page: u32,
    per_page: u32,
}

async fn handle_db_result<T>(
    result: Result<T, Error>,
    tx: Transaction<'_, Postgres>,
) -> Result<T, Error> {
    match result {
        Ok(result) => {
            log::info!("Committing transaction");
            tx.commit().await?;
            Ok(result)
        }
        Err(e) => {
            log::error!("Transaction failed: {:?}", e);
            let r = tx.rollback().await;
            if let Err(e) = r {
                log::error!("Rollback failed: {:?}", e);
            } else {
                log::warn!("Transaction rolled back");
            }
            Err(e)
        }
    }
}

pub trait DbStored: DbStoredInner {
    async fn save(&self, db: &DbClient) -> Result<(), Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = self.inner_save(&mut tx).await;
        handle_db_result(result, tx).await
    }
    async fn update(&self, db: &DbClient) -> Result<(), Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = self.inner_update(&mut tx).await;
        handle_db_result(result, tx).await
    }
    async fn delete(&self, db: &DbClient) -> Result<(), Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = self.inner_delete(&mut tx).await;
        handle_db_result(result, tx).await
    }
    async fn get_by_id(id: &str, db: &DbClient) -> Result<Option<Self>, Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = Self::inner_get_by_id(id, &mut tx).await?;
        handle_db_result(Ok(result), tx).await
    }
    async fn get_all(pagination: DbPagination, db: &DbClient) -> Result<Vec<Self>, Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = Self::inner_get_all(pagination, &mut tx).await?;
        handle_db_result(Ok(result), tx).await
    }
}

impl<T> DbStored for T where T: DbStoredInner {}

pub trait DbStoredDependently: DbStoredDependentlyInner {
    async fn save(&self, db: &DbClient) -> Result<Uuid, Error> {
        let mut tx = db.raw_db().begin().await?;
        let id = self.inner_save(&mut tx).await?;
        tx.commit().await?;
        Ok(id)
    }
}
impl<T> DbStoredDependently for T where T: DbStoredDependentlyInner {}

#[async_trait]
trait DbStoredInner: Sized {
    async fn inner_save(&self, db: &mut Transaction<Postgres>) -> Result<(), Error>;
    async fn inner_update(&self, db: &mut Transaction<Postgres>) -> Result<(), Error>;
    async fn inner_delete(&self, db: &mut Transaction<Postgres>) -> Result<(), Error>;

    async fn inner_get_by_id(
        id: &str,
        db: &mut Transaction<Postgres>,
    ) -> Result<Option<Self>, Error>;

    async fn inner_get_all(
        pagination: DbPagination,
        db: &mut Transaction<Postgres>,
    ) -> Result<Vec<Self>, Error>;
}

#[async_trait]
trait DbStoredDependentlyInner: Sized {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<Uuid, Error>;
}

#[async_trait]
pub(super) trait FromDb: Sized {
    type DbModel;
    async fn from_db_model(
        model: Self::DbModel,
        db: &mut Transaction<Postgres>,
    ) -> Result<Self, Error>;
}

#[async_trait]
impl DbStoredDependentlyInner for NlpMetadata {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<Uuid, Error> {
        let id = sqlx::query!(
            r#"
        INSERT INTO nlp_metadata (generated_in)
        VALUES ($1)
        RETURNING id as "id: Uuid";
        "#,
            self.generated_in
        )
        .fetch_one(&mut **tx)
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl FromDb for NlpMetadata {
    type DbModel = DbNlpMetadata;
    async fn from_db_model(
        model: Self::DbModel,
        db: &mut Transaction<Postgres>,
    ) -> Result<Self, Error> {
        Ok(NlpMetadata {
            generated_in: model.generated_in,
        })
    }
}

#[async_trait]
impl DbStoredDependentlyInner for NlpAnalysis {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<Uuid, Error> {
        let metadata_uuid = self.metadata.inner_save(tx).await?;
        let id = sqlx::query!(
            r#"
            INSERT INTO nlp_analyses (nlp_metadata_id, kind, analysis)
            VALUES ($1, $2, $3)
            RETURNING id as "id: Uuid";
            "#,
            metadata_uuid,
            &self.kind.to_snake_case(),
            serde_json::to_value(&self.results).unwrap() // TODO HANDLE ERROR
        )
        .fetch_one(&mut **tx)
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl FromDb for NlpAnalysis {
    type DbModel = DbNlpAnalysis;
    async fn from_db_model(
        model: Self::DbModel,
        db: &mut Transaction<Postgres>,
    ) -> Result<Self, Error> {
        let nlp_metadata = sqlx::query_as!(
            DbNlpMetadata,
            r#"SELECT * FROM nlp_metadata WHERE id = $1"#,
            model.nlp_metadata_id
        )
        .fetch_one(&mut **db)
        .await?;
        Ok(NlpAnalysis {
            kind: NlpAnalysisKind::from_snake_case(&model.kind).unwrap(),
            results: serde_json::from_value(model.analysis.clone()).unwrap(),
            metadata: NlpMetadata {
                generated_in: nlp_metadata.generated_in,
            },
        })
    }
}

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
        db: &mut Transaction<Postgres>,
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
        .fetch_all(&mut **db)
        .await?;

        let mut analyses_map = HashMap::new();
        for db_analysis in analyses {
            let analysis = NlpAnalysis::from_db_model(db_analysis, db).await?;
            analyses_map.insert(analysis.kind.clone(), analysis);
        }

        Ok(ReportAnalysesMap {
            analyses: analyses_map,
        })
    }
}

#[async_trait]
impl DbStoredDependentlyInner for ReportMetadata {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<Uuid, Error> {
        let id = sqlx::query!(
            r#"
            INSERT INTO report_metadata (report_created_at, report_updated_at)
            VALUES ($1, $2)
            RETURNING id as "id: Uuid";
            "#,
            self.created_at,
            self.updated_at
        )
        .fetch_one(&mut **tx)
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl FromDb for ReportMetadata {
    type DbModel = DbReportMetadata;
    async fn from_db_model(
        model: Self::DbModel,
        db: &mut Transaction<Postgres>,
    ) -> Result<Self, Error> {
        Ok(ReportMetadata {
            created_at: model.report_created_at,
            updated_at: model.report_updated_at,
        })
    }
}

#[async_trait]
impl DbStoredInner for Report {
    async fn inner_save(&self, db: &mut Transaction<Postgres>) -> Result<(), Error> {
        let metadata_uuid = self.metadata.inner_save(db).await?;
        let analyses_uuid = self.analyses_map.inner_save(db).await?;
        let user_uuid = sqlx::query!(
            r#"
            SELECT id as "id: Uuid" FROM users WHERE google_sub = $1
            "#,
            self.user_id
        )
        .fetch_one(&mut **db)
        .await?
        .id;

        sqlx::query!(
            r#"
            INSERT INTO reports (
            user_id, title, description, is_public, metadata_id, analyses_map_id
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            user_uuid,
            self.title,
            self.description,
            self.is_public,
            metadata_uuid,
            analyses_uuid
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }
    async fn inner_update(&self, db: &mut Transaction<Postgres>) -> Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE reports
            SET title = $1, description = $2, is_public = $3
            WHERE display_id = $4
            "#,
            self.title,
            self.description,
            self.is_public,
            self.id
        )
        .execute(&mut **db)
        .await?;
        Ok(())
    }
    async fn inner_delete(&self, db: &mut Transaction<Postgres>) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM reports WHERE display_id = $1
            "#,
            self.id
        )
        .execute(&mut **db)
        .await?;
        Ok(())
    }

    async fn inner_get_by_id(
        id: &str,
        db: &mut Transaction<Postgres>,
    ) -> Result<Option<Self>, Error> {
        let report = sqlx::query_as!(
            DbReport,
            r#"
            SELECT *
            FROM reports
            WHERE display_id = $1
            "#,
            id
        )
        .fetch_optional(&mut **db)
        .await?;

        let report = match report {
            Some(report) => report,
            None => return Ok(None),
        };

        let metadata = {
            let db_metadata = sqlx::query_as!(
                DbReportMetadata,
                r#"
            SELECT *
            FROM report_metadata
            WHERE id = $1
            "#,
                report.metadata_id
            )
            .fetch_one(&mut **db)
            .await?;

            ReportMetadata::from_db_model(db_metadata, db).await?
        };

        let analyses_map = {
            let map = sqlx::query_as!(
                DbReportAnalysesMap,
                r#"
            SELECT *
            FROM report_analyses_maps
            WHERE id = $1
            "#,
                report.analyses_map_id
            )
            .fetch_one(&mut **db)
            .await?;

            ReportAnalysesMap::from_db_model(map, db).await?
        };

        Ok(Some(Report {
            id: report.display_id,
            user_id: report.user_id,
            title: report.title,
            description: report.description,
            is_public: report.is_public,
            metadata,
            analyses_map,
        }))
    }
    async fn inner_get_all(
        pagination: DbPagination,
        db: &mut Transaction<Postgres>,
    ) -> Result<Vec<Self>, Error> {
        todo!()
    }
}

#[async_trait]
impl DbStoredInner for User {
    async fn inner_save(&self, db: &mut Transaction<Postgres>) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO users (
            google_sub, name, given_name, family_name, picture, email, email_verified
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (google_sub) DO UPDATE
            SET name = $2, given_name = $3, family_name = $4, picture = $5, email = $6, email_verified = $7
            "#,
            self.id,
            self.name,
            self.given_name,
            self.family_name,
            self.picture,
            self.email,
            self.email_verified
        )
        .execute(&mut **db)
        .await?;
        Ok(())
    }
    async fn inner_update(&self, db: &mut Transaction<Postgres>) -> Result<(), Error> {
        unimplemented!()
    }
    async fn inner_delete(&self, db: &mut Transaction<Postgres>) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM users WHERE google_sub = $1
            "#,
            self.id
        )
        .execute(&mut **db)
        .await?;
        Ok(())
    }

    async fn inner_get_by_id(
        id: &str,
        db: &mut Transaction<Postgres>,
    ) -> Result<Option<Self>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT google_sub as "id: String", name, given_name, family_name, picture, email, email_verified
            FROM users
            WHERE google_sub = $1
            "#,
            id
        )
        .fetch_optional(&mut **db)
        .await?;
        Ok(user)
    }

    async fn inner_get_all(
        pagination: DbPagination,
        db: &mut Transaction<Postgres>,
    ) -> Result<Vec<Self>, Error> {
        todo!()
    }
}
