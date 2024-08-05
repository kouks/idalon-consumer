use crate::models::generic::*;
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
    pub verified_only: bool,
    pub season: usize,
    pub order_by: String,
    pub order_direction: String,
}

impl Default for NightFilters {
    fn default() -> Self {
        NightFilters {
            offset: 0,
            limit: 10,
            verified_only: true,
            season: 3,
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

    fn resource_url() -> String {
        String::from("https://api.idalon.com/v2/nights")
    }

    async fn find_one(uuid: &str) -> Self {
        Self::fetch_one(format!("{}/{}", Self::resource_url(), uuid))
            .await
            .unwrap()
    }

    async fn find_many(filters: NightFilters) -> Collection<Self> {
        Self::fetch_many(Self::resource_url(), filters)
            .await
            .unwrap()
    }
}
