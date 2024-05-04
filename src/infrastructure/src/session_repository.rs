use application::session_service::SessionRepository;
use async_trait::async_trait;
use domain::session::Session;
use sqlx::PgPool;

pub struct SessionRepositoryImpl {
    pool: PgPool,
}

impl SessionRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionRepository for SessionRepositoryImpl {
    async fn create(&self, session: Session) -> Result<Session, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO session (id, session_token, user_id, expires)
            VALUES ($1, $2, $3, $4)
            "#,
            session.id,
            session.session_token,
            session.user_id,
            session.expires
        )
        .execute(&self.pool)
        .await?;

        Ok(session)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Session>, sqlx::Error> {
        let session = sqlx::query_as!(
            Session,
            r#"
            SELECT id, session_token, user_id, expires
            FROM session
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Option<Session>, sqlx::Error> {
        let session = sqlx::query_as!(
            Session,
            r#"
            SELECT id, session_token, user_id, expires
            FROM session
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    async fn find_by_session_token(
        &self,
        session_token: &str,
    ) -> Result<Option<Session>, sqlx::Error> {
        let session = sqlx::query_as!(
            Session,
            r#"
            SELECT id, session_token, user_id, expires
            FROM session
            WHERE session_token = $1
            "#,
            session_token
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    async fn delete(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM session
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
