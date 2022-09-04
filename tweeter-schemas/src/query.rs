use serde::{Deserialize, Serialize};

use crate::resource_type::ResourceType;

#[derive(Deserialize, Serialize)]
pub struct Include {
    pub include: Option<ResourceType>,
}

#[derive(Deserialize, Clone, Serialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Order {
    Desc,
    Asc,
}

pub const DEFAULT_PAGE_LIMIT: u64 = 10;
pub const DEFAULT_PAGE_NUMBER: u64 = 0;
pub const DEFAULT_PAGE_ORDER: Order = Order::Desc;

fn default_page_limit() -> u64 {
    DEFAULT_PAGE_LIMIT
}

fn default_page_order() -> Order {
    DEFAULT_PAGE_ORDER
}

fn default_page_number() -> u64 {
    DEFAULT_PAGE_NUMBER
}

#[derive(Deserialize, Clone, Serialize, PartialEq, Debug)]
pub struct Pagination {
    #[serde(rename = "page[limit]", default = "default_page_limit")]
    pub limit: u64,
    #[serde(rename = "page[order]", default = "default_page_order")]
    pub order: Order,
    #[serde(rename = "page[number]", default = "default_page_number")]
    pub number: u64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: default_page_limit(),
            order: default_page_order(),
            number: default_page_number(),
        }
    }
}
