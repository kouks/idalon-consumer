use crate::models::generic::*;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct NightInRun {
    pub uuid: String,
    pub scope: String,
    pub verified: bool,
    pub season: usize,
    pub squad_size: usize,
    pub created_at: String,
    pub users: Vec<SquadMember>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Run {
    pub uuid: String,
    pub verified: bool,
    pub last_hydrolyst_limb_break_time: Option<f32>,
    pub real_time: Option<f32>,
    pub extraction_time: f32,
    pub load_time: Option<f32>,
    pub median_limb_break_time: Option<f32>,
    pub night: NightInRun,
    pub eidolons: Option<Vec<Eidolon>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct RunFilters {
    pub offset: usize,
    pub limit: usize,
    pub verified_only: Option<bool>,
    pub season: Option<usize>,
    pub order_by: String,
    pub order_direction: String,
}

impl Default for RunFilters {
    fn default() -> Self {
        RunFilters {
            offset: 0,
            limit: 10,
            verified_only: None,
            season: None,
            order_by: String::from("createdAt"),
            order_direction: String::from("desc"),
        }
    }
}

impl RunFilters {
    pub fn leaderboard() -> Self {
        Self {
            offset: 0,
            limit: 50,
            season: Some(3),
            verified_only: Some(true),
            order_by: String::from("realTime"),
            order_direction: String::from("asc"),
        }
    }
}

impl Paginable for RunFilters {
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

impl Model for Run {
    type Filters = RunFilters;

    fn resource_url() -> Url {
        Url::parse("https://api.idalon.com/v2/runs").unwrap()
    }
}
