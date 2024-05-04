use application::{session_service::SessionService, user_service::UserService};
use axum::{routing::post, Extension, Router};
use controller::user_controller;
use infrastructure::{
    db, session_repository::SessionRepositoryImpl, user_repository::UserRepositoryImpl,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let pool = db::establish_connection()
        .await
        .expect("Failed to connect to the database");

    let user_repository = UserRepositoryImpl::new(pool.clone());
    let user_service = UserService::new(Arc::new(user_repository));
    let user_service = Arc::new(user_service);

    let session_repository = SessionRepositoryImpl::new(pool.clone());
    let session_service = SessionService::new(Arc::new(session_repository));
    let session_service = Arc::new(session_service);

    let app = Router::new()
        .route("/users", post(user_controller::create_user))
        // 他のルート...
        .layer(Extension(user_service))
        .layer(Extension(session_service));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
