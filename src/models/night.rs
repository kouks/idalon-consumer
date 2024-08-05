use crate::models::generic::*;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Night {
    pub uuid: String,
    pub average_real_time: f32,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct NightFilters {
    pub offset: usize,
    pub limit: usize,
    pub verified_only: Option<bool>,
    pub season: Option<usize>,
    pub order_by: String,
    pub order_direction: String,
}

impl Default for NightFilters {
    fn default() -> Self {
        NightFilters {
            offset: 0,
            limit: 10,
            verified_only: None,
            season: None,
            order_by: String::from("createdAt"),
            order_direction: String::from("desc"),
        }
    }
}

impl NightFilters {
    pub fn leaderboard() -> Self {
        Self {
            offset: 0,
            limit: 25,
            season: Some(3),
            verified_only: Some(true),
            order_by: String::from("averageRealTime"),
            order_direction: String::from("asc"),
        }
    }
}

impl Paginable for NightFilters {
    fn set_page(&mut self, page: usize) {
        self.offset = page * self.limit
    }

    fn get_page(&self) -> usize {
        self.offset / self.limit
    }
}

impl Model for Night {
    type Filters = NightFilters;

    fn resource_url() -> Url {
        Url::parse("https://api.idalon.com/v2/nights").unwrap()
    }
}
