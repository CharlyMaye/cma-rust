use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


#[derive(Deserialize)]
pub struct LoginCredentials {
    pub user: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub success: bool,
}

#[derive(Clone)]
pub struct Session {
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}