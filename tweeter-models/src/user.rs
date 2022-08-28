#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone)]
pub struct User {
    pub public_key: String,
    pub username: String,
    pub image_url: Option<String>,
}
