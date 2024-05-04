use std::sync::Arc;

// application/src/session_service.rs
use async_trait::async_trait;
use domain::session::Session;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: Session) -> Result<Session, sqlx::Error>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Session>, sqlx::Error>;
    async fn find_by_user_id(&self, user_id: &str) -> Result<Option<Session>, sqlx::Error>;
    async fn find_by_session_token(
        &self,
        session_token: &str,
    ) -> Result<Option<Session>, sqlx::Error>;
    async fn delete(&self, id: &str) -> Result<(), sqlx::Error>;
}

#[derive(Clone)]
pub struct SessionService {
    repository: Arc<dyn SessionRepository + Send + Sync>,
}

impl SessionService {
    pub fn new(repository: Arc<dyn SessionRepository + Send + Sync>) -> Self {
        Self { repository }
    }
    pub async fn create_session(&self, session: Session) -> Result<Session, sqlx::Error> {
        self.repository.create(session).await
    }

    pub async fn find_session_by_id(&self, id: &str) -> Result<Option<Session>, sqlx::Error> {
        self.repository.find_by_id(id).await
    }

    pub async fn find_session_by_session_token(
        &self,
        session_token: &str,
    ) -> Result<Option<Session>, sqlx::Error> {
        self.repository.find_by_session_token(session_token).await
    }

    pub async fn find_session_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Option<Session>, sqlx::Error> {
        self.repository.find_by_user_id(user_id).await
    }

    pub async fn delete_session(&self, id: &str) -> Result<(), sqlx::Error> {
        self.repository.delete(id).await
    }
}
