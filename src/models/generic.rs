use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::error::IdalonError;
use reqwest::Url;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio_stream::Stream;

#[derive(Deserialize, Debug)]
pub struct Collection<T> {
    pub total: usize,
    pub items: Vec<T>,
}

pub struct Paginator<T: Model> {
    filters: T::Filters,
    future: Option<Pin<Box<dyn Future<Output = Result<Collection<T>, IdalonError>>>>>,
    was_last: bool,
}

pub trait Paginable {
    fn set_page(&mut self, page: usize);
    fn get_page(&self) -> usize;
    fn get_page_size(&self) -> usize;
}

impl<T: Model> Paginator<T> {
    pub fn new(filters: T::Filters) -> Self {
        Paginator {
            filters,
            future: None,
            was_last: false,
        }
    }
}

impl<T: Model> Stream for Paginator<T>
where
    T::Filters: Unpin + 'static,
{
    type Item = Result<Collection<T>, IdalonError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.was_last {
            return Poll::Ready(None);
        }

        if self.future.is_none() {
            let filters = self.filters.clone();

            let next_page = self.filters.get_page() + 1;

            self.filters.set_page(next_page);

            self.future = Some(Box::pin(async { T::find_many(filters).await }))
        }

        let future = self
            .future
            .as_mut()
            .expect("Option is not None as checked before.");

        match future.as_mut().poll(cx) {
            Poll::Ready(result) => {
                self.future = None;

                if result.is_err() {
                    return Poll::Ready(None);
                }

                let data = result
                    .as_ref()
                    .expect("Result is not Err as checked before.");
                let polled = (self.filters.get_page() - 1) * self.filters.get_page_size();

                if data.total <= polled {
                    self.was_last = true
                }

                match data.items.len() {
                    0 => Poll::Ready(None),
                    _ => Poll::Ready(Some(result)),
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait Model: DeserializeOwned {
    type Filters: Paginable + Serialize + Clone;

    fn resource_url() -> Url;

    async fn find_many(filters: Self::Filters) -> Result<Collection<Self>, IdalonError> {
        Self::fetch_many(Self::resource_url(), filters).await
    }

    async fn find_one(uuid: &str) -> Result<Self, IdalonError> {
        let url = Url::parse(&format!("{}/{}", Self::resource_url().as_str(), uuid)).unwrap();

        Self::fetch_one(url).await
    }

    fn paginate(filters: Self::Filters) -> Paginator<Self> {
        Paginator::new(filters)
    }

    async fn fetch_many(url: Url, filters: Self::Filters) -> Result<Collection<Self>, IdalonError> {
        let client = reqwest::Client::new();

        let response = client
            .get(url.as_str())
            .query(&filters)
            .send()
            .await
            .map_err(crate::error::fetch)?;

        match response.status().as_u16() {
            x if x < 400 => Ok(response
                .json::<Collection<Self>>()
                .await
                .map_err(crate::error::parse)?),
            _ => Err(crate::error::IdalonError::new(
                "Request returned an error status code.",
            )),
        }
    }

    async fn fetch_one(url: Url) -> Result<Self, IdalonError> {
        let client = reqwest::Client::new();

        let response = client
            .get(url.as_str())
            .send()
            .await
            .map_err(crate::error::fetch)?;

        match response.status().as_u16() {
            x if x < 400 => Ok(response.json::<Self>().await.map_err(crate::error::parse)?),
            _ => Err(crate::error::IdalonError::new(
                "Request returned an error status code.",
            )),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SquadMember {
    pub uuid: Option<String>,
    pub ign: Option<String>,
    pub role: Option<String>,
    pub scope: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Eidolon {
    pub result: String,
    pub spawn_delay: Option<f32>,
    pub spawn_animation_time: Option<f32>,
    pub first_limb_break_time: Option<f32>,
    pub last_limb_break_time: Option<f32>,
    pub median_limb_break_time: Option<f32>,
    pub limb_break_times: Vec<f32>,
    pub shrine_time: Option<f32>,
    pub shard_insertion_times: Vec<f32>,
    pub capshot_time: Option<f32>,
}
