use application::user_service::UserRepository;
use async_trait::async_trait;
use domain::user::User;
use sqlx::PgPool;

pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

unsafe impl Send for UserRepositoryImpl {}
unsafe impl Sync for UserRepositoryImpl {}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: User) -> Result<User, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, name, email, email_verified, image)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user.id,
            user.name,
            user.email,
            user.email_verified,
            user.image
        )
        .execute(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, email, email_verified, image
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, email, email_verified, image
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update(&self, user: User) -> Result<User, sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET name = $2, email = $3, email_verified = $4, image = $5
            WHERE id = $1
            "#,
            user.id,
            user.name,
            user.email,
            user.email_verified,
            user.image
        )
        .execute(&self.pool)
        .await?;

        Ok(user)
    }

    async fn delete(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
