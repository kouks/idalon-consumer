use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::error::IdalonError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio_stream::Stream;

#[derive(Deserialize, Debug)]
pub struct Collection<T> {
    pub total: usize,
    pub items: Vec<T>,
}

pub struct Paginator<T: Model> {
    filters: T::Filters,
    future: Option<Pin<Box<dyn Future<Output = Collection<T>>>>>,
}

pub trait Paginable {
    fn set_page(&mut self, page: usize);
    fn get_page(&self) -> usize;
}

impl<T: Model> Paginator<T> {
    pub fn new(filters: T::Filters) -> Self {
        Paginator {
            filters: filters,
            future: None,
        }
    }
}

impl<T: Model> Stream for Paginator<T>
where
    Paginator<T>: Unpin,
    T::Filters: Unpin,
{
    type Item = Collection<T>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut filters = self.filters.clone();

        filters.set_page(self.filters.get_page() + 1);

        if let None = self.future {
            let future = async {
                // let filters = T::Filters::default();

                T::fetch_many(T::resource_url(), filters).await.unwrap()
            };

            self.future = Some(Box::pin(future))
        }

        let future = self.future.as_mut().unwrap();

        match future.as_mut().poll(cx) {
            Poll::Ready(data) => match data.items.len() {
                0 => Poll::Ready(None),
                _ => Poll::Ready(Some(data)),
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

pub trait Model: DeserializeOwned {
    type Filters: Paginable + Serialize + Default + Clone;

    // Get the resource URL;
    fn resource_url() -> String;

    // List all resources of type T with the provided filters.
    #[allow(async_fn_in_trait)]
    async fn find_many(filters: Self::Filters) -> Collection<Self>;

    // Fetch a detail of a resource with the specified UUID.
    #[allow(async_fn_in_trait)]
    async fn find_one(uuid: &str) -> Self;

    // List all available resources
    #[allow(async_fn_in_trait)]
    async fn all() -> Collection<Self> {
        Self::find_many(Self::Filters::default()).await
    }

    fn paginate(filters: Self::Filters) -> Paginator<Self> {
        Paginator::new(filters)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_many(
        url: String,
        filters: Self::Filters,
    ) -> Result<Collection<Self>, IdalonError> {
        let client = reqwest::Client::new();

        let response = client
            .get(&url)
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

    #[allow(async_fn_in_trait)]
    async fn fetch_one(url: String) -> Result<Self, IdalonError> {
        let client = reqwest::Client::new();

        let response = client.get(&url).send().await.map_err(crate::error::fetch)?;

        match response.status().as_u16() {
            x if x < 400 => Ok(response.json::<Self>().await.map_err(crate::error::parse)?),
            _ => Err(crate::error::IdalonError::new(
                "Request returned an error status code.",
            )),
        }
    }
}
