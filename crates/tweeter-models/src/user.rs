#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct User {
    pub public_key: String,
    pub username: String,
    pub image_url: Option<String>,
}
