use crate::api::auth::google::User;
use crate::db::db_client::DbClient;
use crate::db::model::{
    DbNlpAnalysis, DbNlpMetadata, DbReport, DbReportAnalysesMap, DbReportMetadata,
};
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::{NlpAnalysis, NlpMetadata};
use crate::nlp::report::{Report, ReportAnalysesMap, ReportMetadata};
use axum::async_trait;
use sqlx::Error;
use std::collections::HashMap;
use uuid::Uuid;

pub struct DbPagination {
    page: u32,
    per_page: u32,
}

/// Implemented for structs that can be saved to our database
#[async_trait]
pub trait DbStored: Sized {
    async fn save(&self, db: &DbClient) -> Result<(), Error>;
    async fn update(&self, db: &DbClient) -> Result<(), Error>;
    async fn delete(&self, db: &DbClient) -> Result<(), Error>;

    async fn get_by_id(id: &str, db: &DbClient) -> Result<Option<Self>, Error>;
    async fn get_all(pagination: DbPagination, db: &DbClient) -> Result<Vec<Self>, Error>;
}

#[async_trait]
pub trait DbStoredDependently: Sized {
    async fn save(&self, db: &DbClient) -> Result<Uuid, Error>;
}

#[async_trait]
pub(super) trait FromDb: Sized {
    type DbModel;
    async fn from_db_model(model: Self::DbModel, db: &DbClient) -> Result<Self, Error>;
}

#[async_trait]
impl DbStoredDependently for NlpMetadata {
    async fn save(&self, db: &DbClient) -> Result<Uuid, Error> {
        let id = sqlx::query!(
            r#"
            INSERT INTO nlp_metadata (generated_in)
            VALUES ($1)
            RETURNING id as "id: Uuid";
            "#,
            self.generated_in
        )
        .fetch_one(db.raw_db())
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl FromDb for NlpMetadata {
    type DbModel = DbNlpMetadata;
    async fn from_db_model(db_nlp_metadata: Self::DbModel, db: &DbClient) -> Result<Self, Error> {
        Ok(NlpMetadata {
            generated_in: db_nlp_metadata.generated_in,
        })
    }
}

#[async_trait]
impl DbStoredDependently for NlpAnalysis {
    async fn save(&self, db: &DbClient) -> Result<Uuid, Error> {
        let metadata_uuid = self.metadata.save(db).await?;
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
        .fetch_one(db.raw_db())
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl FromDb for NlpAnalysis {
    type DbModel = DbNlpAnalysis;
    async fn from_db_model(db_analysis: Self::DbModel, db: &DbClient) -> Result<Self, Error> {
        let nlp_metadata = sqlx::query_as!(
            DbNlpMetadata,
            r#"SELECT * FROM nlp_metadata WHERE id = $1"#,
            db_analysis.nlp_metadata_id
        )
        .fetch_one(db.raw_db())
        .await?;
        Ok(NlpAnalysis {
            kind: NlpAnalysisKind::from_snake_case(&db_analysis.kind).unwrap(),
            results: serde_json::from_value(db_analysis.analysis.clone()).unwrap(),
            metadata: NlpMetadata {
                generated_in: nlp_metadata.generated_in,
            },
        })
    }
}

#[async_trait]
impl DbStoredDependently for ReportAnalysesMap {
    async fn save(&self, db: &DbClient) -> Result<Uuid, Error> {
        let save_futures = self
            .analyses
            .iter()
            .map(|(kind, analysis)| async move {
                let id = analysis.save(db).await?;
                Ok::<(NlpAnalysisKind, Uuid), Error>((kind.clone(), id))
            })
            .collect::<Vec<_>>();

        // Await all futures concurrently, then collect them into a hashmap.
        // If any one of them fails, return the error
        let map: HashMap<NlpAnalysisKind, Uuid> = futures::future::join_all(save_futures)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|(kind, id): (NlpAnalysisKind, Uuid)| (kind.clone(), id))
            .collect();

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
        .fetch_one(db.raw_db())
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl FromDb for ReportAnalysesMap {
    type DbModel = DbReportAnalysesMap;
    async fn from_db_model(db_analyses_map: Self::DbModel, db: &DbClient) -> Result<Self, Error> {
        let analyses = sqlx::query_as!(
            DbNlpAnalysis,
            r#"
            SELECT *
            FROM nlp_analyses
            WHERE nlp_metadata_id IN (
                $1, $2, $3, $4, $5, $6, $7, $8
            )
            "#,
            db_analyses_map.clickbait_id,
            db_analyses_map.hate_speech_id,
            db_analyses_map.keywords_id,
            db_analyses_map.language_id,
            db_analyses_map.politics_id,
            db_analyses_map.sarcasm_id,
            db_analyses_map.sentiment_id,
            db_analyses_map.spam_id
        )
        .fetch_all(db.raw_db())
        .await?;

        let mapped_analyses_fut = analyses
            .into_iter()
            .map(|db_analysis| NlpAnalysis::from_db_model(db_analysis, db));

        let analyses_map: HashMap<_, _> = futures::future::join_all(mapped_analyses_fut)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|analysis| (analysis.kind.clone(), analysis))
            .collect();

        Ok(ReportAnalysesMap {
            analyses: analyses_map,
        })
    }
}

#[async_trait]
impl DbStoredDependently for ReportMetadata {
    async fn save(&self, db: &DbClient) -> Result<Uuid, Error> {
        let id = sqlx::query!(
            r#"
            INSERT INTO report_metadata (report_created_at, report_updated_at)
            VALUES ($1, $2)
            RETURNING id as "id: Uuid";
            "#,
            self.created_at,
            self.updated_at
        )
        .fetch_one(db.raw_db())
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl FromDb for ReportMetadata {
    type DbModel = DbReportMetadata;
    async fn from_db_model(
        db_report_metadata: Self::DbModel,
        db: &DbClient,
    ) -> Result<Self, Error> {
        Ok(ReportMetadata {
            created_at: db_report_metadata.report_created_at,
            updated_at: db_report_metadata.report_updated_at,
        })
    }
}

#[async_trait]
impl DbStored for Report {
    async fn save(&self, db: &DbClient) -> Result<(), Error> {
        let metadata_uuid = self.metadata.save(db).await?;
        let analyses_uuid = self.analyses_map.save(db).await?;
        let user_uuid = sqlx::query!(
            r#"
            SELECT id as "id: Uuid" FROM users WHERE google_sub = $1
            "#,
            self.user_id
        )
        .fetch_one(db.raw_db())
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
        .execute(db.raw_db())
        .await?;

        Ok(())
    }
    async fn update(&self, db: &DbClient) -> Result<(), Error> {
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
        .execute(db.raw_db())
        .await?;
        Ok(())
    }
    async fn delete(&self, db: &DbClient) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM reports WHERE display_id = $1
            "#,
            self.id
        )
        .execute(db.raw_db())
        .await?;
        Ok(())
    }

    async fn get_by_id(id: &str, db: &DbClient) -> Result<Option<Self>, Error> {
        let report = sqlx::query_as!(
            DbReport,
            r#"
            SELECT *
            FROM reports
            WHERE display_id = $1
            "#,
            id
        )
        .fetch_optional(db.raw_db())
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
            .fetch_one(db.raw_db())
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
            .fetch_one(db.raw_db())
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
    async fn get_all(pagination: DbPagination, db: &DbClient) -> Result<Vec<Self>, Error> {
        todo!()
    }
}

#[async_trait]
impl DbStored for User {
    async fn save(&self, db: &DbClient) -> Result<(), Error> {
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
        .execute(db.raw_db())
        .await?;
        Ok(())
    }
    async fn update(&self, db: &DbClient) -> Result<(), Error> {
        unimplemented!()
    }
    async fn delete(&self, db: &DbClient) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM users WHERE google_sub = $1
            "#,
            self.id
        )
        .execute(db.raw_db())
        .await?;
        Ok(())
    }

    async fn get_by_id(id: &str, db: &DbClient) -> Result<Option<Self>, Error> {
        todo!()
    }

    async fn get_all(pagination: DbPagination, db: &DbClient) -> Result<Vec<Self>, Error> {
        todo!()
    }
}
