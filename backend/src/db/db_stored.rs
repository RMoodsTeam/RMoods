use crate::api::auth::google::User;
use crate::db::model::{DbReport, DbUser};
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::{NlpAnalysis, NlpMetadata};
use crate::nlp::report::{Report, ReportAnalysesMap, ReportMetadata};
use axum::async_trait;
use sqlx::{Error, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

/// Implemented for structs that can be saved to our database
#[async_trait]
pub trait DbStored {
    type DbStruct;
    async fn save(&self, db: &PgPool) -> Result<(), sqlx::Error>;
    async fn update(&self, db: &PgPool) -> Result<(), sqlx::Error>;
    async fn delete(&self, db: &PgPool) -> Result<(), sqlx::Error>;
}

#[async_trait]
pub trait DbStoredDependently {
    async fn save(&self, db: &PgPool) -> Result<Uuid, sqlx::Error>;
}

#[async_trait]
impl DbStoredDependently for NlpMetadata {
    async fn save(&self, db: &PgPool) -> Result<Uuid, Error> {
        let id = sqlx::query!(
            r#"
            INSERT INTO nlp_metadata (generated_in)
            VALUES ($1)
            RETURNING id as "id: Uuid";
            "#,
            self.generated_in
        )
        .fetch_one(db)
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl DbStoredDependently for NlpAnalysis {
    async fn save(&self, db: &PgPool) -> Result<Uuid, Error> {
        let metadata_uuid = self.metadata.save(db).await?;
        let id = sqlx::query!(
            r#"
            INSERT INTO nlp_analyses (nlp_metadata_id, kind, analysis)
            VALUES ($1, $2, $3)
            RETURNING id as "id: Uuid";
            "#,
            metadata_uuid,
            &self.kind as &NlpAnalysisKind,
            serde_json::to_value(&self.results).unwrap()
        )
        .fetch_one(db)
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl DbStoredDependently for ReportAnalysesMap {
    async fn save(&self, db: &PgPool) -> Result<Uuid, Error> {
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
            map.get(&A::Clickbait).copied(),
            map.get(&A::HateSpeech).copied(),
            map.get(&A::Keywords).copied(),
            map.get(&A::Language).copied(),
            map.get(&A::Politics).copied(),
            map.get(&A::Sarcasm).copied(),
            map.get(&A::Sentiment).copied(),
            map.get(&A::Spam).copied(),
        )
        .fetch_one(db)
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl DbStoredDependently for ReportMetadata {
    async fn save(&self, db: &PgPool) -> Result<Uuid, Error> {
        let id = sqlx::query!(
            r#"
            INSERT INTO report_metadata (report_created_at, report_updated_at)
            VALUES ($1, $2)
            RETURNING id as "id: Uuid";
            "#,
            self.created_at,
            self.updated_at
        )
        .fetch_one(db)
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl DbStored for Report {
    type DbStruct = DbReport;
    async fn save(&self, db: &PgPool) -> Result<(), Error> {
        let metadata_uuid = self.metadata.save(db).await?;
        let analyses_uuid = self.analyses_map.save(db).await?;
        let user_uuid = sqlx::query!(
            r#"
            SELECT id as "id: Uuid" FROM users WHERE google_sub = $1
            "#,
            self.user_id
        )
        .fetch_one(db)
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
        .execute(db)
        .await?;

        Ok(())
    }
    async fn update(&self, db: &PgPool) -> Result<(), Error> {
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
        .execute(db)
        .await?;
        Ok(())
    }
    async fn delete(&self, db: &PgPool) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM reports WHERE display_id = $1
            "#,
            self.id
        )
        .execute(db)
        .await?;
        Ok(())
    }
}

#[async_trait]
impl DbStored for User {
    type DbStruct = DbUser;
    async fn save(&self, db: &PgPool) -> Result<(), Error> {
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
        .execute(db)
        .await?;
        Ok(())
    }
    async fn update(&self, db: &PgPool) -> Result<(), Error> {
        unimplemented!()
    }
    async fn delete(&self, db: &PgPool) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM users WHERE google_sub = $1
            "#,
            self.id
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
