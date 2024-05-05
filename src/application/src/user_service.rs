use async_trait::async_trait;
use domain::user::User;
use mockall::automock;
use std::sync::Arc;
#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: User) -> Result<User, sqlx::Error>;
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
    async fn update(&self, user: User) -> Result<User, sqlx::Error>;
    async fn delete(&self, id: &str) -> Result<(), sqlx::Error>;
}

#[derive(Clone)]
pub struct UserService {
    repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_user(&self, user: User) -> Result<User, sqlx::Error> {
        self.repository.create(user).await
    }

    pub async fn find_user_by_id(&self, id: &str) -> Result<Option<User>, sqlx::Error> {
        self.repository.find_by_id(id).await
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        self.repository.find_by_email(email).await
    }

    pub async fn update_user(&self, user: User) -> Result<User, sqlx::Error> {
        self.repository.update(user).await
    }

    pub async fn delete_user(&self, id: &str) -> Result<(), sqlx::Error> {
        self.repository.delete(id).await
    }
}
