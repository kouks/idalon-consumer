use crate::models::generic::*;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct RunInNight {
    pub uuid: String,
    pub verified: bool,
    pub last_hydrolyst_limb_break_time: Option<f32>,
    pub real_time: Option<f32>,
    pub extraction_time: f32,
    pub load_time: Option<f32>,
    pub median_limb_break_time: Option<f32>,
    pub eidolons: Option<Vec<Eidolon>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct VerificationInNight {
    pub uuid: String,
    pub status: String,
    pub video_url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Night {
    pub uuid: String,
    pub scope: String,
    pub verified: bool,
    pub season: usize,
    pub squad_size: usize,
    pub duration: f32,
    pub average_extraction_time: f32,
    pub average_real_time: Option<f32>,
    pub average_last_hydrolyst_limb_break_time: Option<f32>,
    pub active_time: Option<f32>,
    pub last_eidolon_capshot_time: Option<f32>,
    pub median_limb_break_time: Option<f32>,
    pub captured_hydrolysts_count: usize,
    pub captured_gantulysts_count: usize,
    pub captured_teralysts_count: usize,
    pub description: String,
    pub created_at: String,
    pub users: Vec<SquadMember>,
    pub runs: Option<Vec<RunInNight>>,
    pub verification: Option<VerificationInNight>,
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

    fn get_page_size(&self) -> usize {
        self.limit
    }
}

impl Model for Night {
    type Filters = NightFilters;

    fn resource_url() -> Url {
        Url::parse("https://api.idalon.com/v2/nights").unwrap()
    }
}
