// This is free and unencumbered software released into the public domain.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination<T = serde_json::Value> {
    pub entries: Vec<T>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Root<T = serde_json::Value> {
    pub infos: Vec<T>,
}
