#[derive(Debug)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<chrono::NaiveDateTime>,
    pub image: Option<String>,
}
