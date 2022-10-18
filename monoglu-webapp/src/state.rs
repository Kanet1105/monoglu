#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserInfo {
    id: String,
    token: String,
}
