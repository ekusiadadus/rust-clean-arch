use application::user_service::UserService;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};
use domain::user::User;
use serde::{Deserialize, Serialize};

use thiserror::Error;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(url)]
    pub image: Option<String>,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<chrono::NaiveDateTime>,
    pub image: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Error, Serialize)]
pub enum CreateUserError {
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for CreateUserError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            CreateUserError::Validation(_) => StatusCode::BAD_REQUEST,
            CreateUserError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = Json(CreateUserResponse {
            id: "".to_string(),
            name: None,
            email: None,
            email_verified: None,
            image: None,
            error: Some(self.to_string()),
        });
        (status_code, body).into_response()
    }
}

pub async fn create_user(
    Extension(user_service): Extension<UserService>,
    Json(req): Json<CreateUserRequest>,
) -> impl IntoResponse {
    if let Err(err) = req.validate() {
        return CreateUserError::Validation(err.to_string()).into_response();
    }
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
                error: None,
            }),
        )
            .into_response(),
        Err(_) => CreateUserError::Internal.into_response(),
    }
}
