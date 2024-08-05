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
    page: usize,
    future: Option<Pin<Box<dyn Future<Output = Result<Collection<T>, IdalonError>>>>>,
}

pub trait Paginable {
    fn set_page(&mut self, page: usize);
    fn get_page(&self) -> usize;
}

impl<T: Model> Paginator<T> {
    pub fn new(filters: T::Filters) -> Self {
        Paginator {
            filters,
            page: 0,
            future: None,
        }
    }
}

impl<T: Model> Stream for Paginator<T>
where
    T::Filters: Unpin + 'static,
{
    type Item = Collection<T>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let None = self.future {
            let mut filters = self.filters.clone();

            filters.set_page(self.page);

            self.page = self.page + 1;

            self.future = Some(Box::pin(async { T::find_many(filters).await }))
        }

        let future = self.future.as_mut().unwrap();

        match future.as_mut().poll(cx) {
            Poll::Ready(result) => {
                self.future = None;

                if result.is_err() {
                    return Poll::Ready(None);
                }

                let data = result.unwrap();

                match data.items.len() {
                    0 => Poll::Ready(None),
                    _ => Poll::Ready(Some(data)),
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
