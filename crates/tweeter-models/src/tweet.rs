#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Clone, Debug, PartialEq)]
pub struct Tweet {
    pub id: i64,
    pub text: String,
    pub timestamp: i32,
    pub user_id: String,
    pub signature: String,
    pub hash: Option<String>,
    pub previous_id: Option<i64>,
}

impl Default for Tweet {
    fn default() -> Self {
        Self {
            id: Default::default(),
            text: Default::default(),
            timestamp: Default::default(),
            user_id: Default::default(),
            signature: Default::default(),
            hash: Default::default(),
            previous_id: Default::default(),
        }
    }
}
