#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Tweet {
    pub id: i64,
    pub text: String,
    pub timestamp: i32,
    pub user_id: String,
    pub signature: String,
    pub hash: Option<String>,
    pub previous_id: Option<i64>,
}
