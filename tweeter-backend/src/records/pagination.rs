use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Order {
    Desc,
    Asc,
}

impl Into<sea_query::Order> for Order {
    fn into(self) -> sea_query::Order {
        match self {
            Self::Asc => sea_query::Order::Asc,
            Self::Desc => sea_query::Order::Desc,
        }
    }
}

pub const DEFAULT_PAGE_LIMIT: u64 = 10;
pub const DEAFAULT_PAGE_NUMBER: u64 = 0;

fn default_page_limit() -> u64 {
    DEFAULT_PAGE_LIMIT
}

fn default_page_order() -> Order {
    Order::Desc
}

fn default_page_number() -> u64 {
    DEAFAULT_PAGE_NUMBER
}

#[derive(Deserialize)]
pub struct Pagination {
    #[serde(rename = "page[limit]", default = "default_page_limit")]
    pub limit: u64,
    #[serde(rename = "page[order]", default = "default_page_order")]
    pub order: Order,
    #[serde(rename = "page[number]", default = "default_page_number")]
    pub number: u64,
}
