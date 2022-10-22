use serde::Deserialize;

#[derive(Debug, serde::Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub password: String,
}
