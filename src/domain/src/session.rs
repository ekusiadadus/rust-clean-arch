pub struct Session {
    pub id: String,
    pub session_token: String,
    pub user_id: String,
    pub expires: chrono::NaiveDateTime,
}
