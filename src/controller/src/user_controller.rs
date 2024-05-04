use application::user_service::UserService;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};
use domain::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<chrono::NaiveDateTime>,
    pub image: Option<String>,
}

pub async fn create_user(
    Json(req): Json<CreateUserRequest>,
    Extension(user_service): Extension<UserService>,
) -> impl IntoResponse {
    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        name: req.name,
        email: req.email,
        email_verified: None,
        image: None,
    };

    match user_service.create_user(user).await {
        Ok(user) => (
            StatusCode::CREATED,
            Json(CreateUserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                email_verified: user.email_verified,
                image: user.image,
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CreateUserResponse {
                id: "".to_string(),
                name: None,
                email: None,
                email_verified: None,
                image: None,
            }),
        ),
    }
}
