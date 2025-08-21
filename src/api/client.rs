// This is free and unencumbered software released into the public domain.

use crate::api::Error;
use crate::api::models::{Pagination, Root};
use reqwest::{Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder, Url};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Client {
    client: ReqwestClient,
}

impl Client {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            client: ReqwestClientBuilder::new()
                .user_agent("asimov-luma-module")
                .build()
                .unwrap(),
        }
    }

    pub async fn get_featured_calendars(&self) -> Result<Root> {
        let url = Url::parse("https://api.lu.ma/calendar/get-featured-calendars").unwrap();
        let response = self.client.get(url).send().await?;
        response.error_for_status_ref()?;
        Ok(response.json::<Root>().await?)
    }

    pub async fn list_categories(&self) -> Result<Pagination> {
        let url =
            Url::parse("https://api.lu.ma/discover/category/list-categories?pagination_limit=20")
                .unwrap();
        let response = self.client.get(url).send().await?;
        response.error_for_status_ref()?;
        Ok(response.json::<Pagination>().await?)
    }

    pub async fn list_places(&self) -> Result<Root> {
        let url = Url::parse("https://api.lu.ma/discover/list-places").unwrap();
        let response = self.client.get(url).send().await?;
        response.error_for_status_ref()?;
        Ok(response.json::<Root>().await?)
    }

    pub async fn get_category_by_id<S>(&self, id: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = Url::parse(&format!(
            "https://api.lu.ma/discover/category/get-page?discover_category_api_id={}",
            id.as_ref()
        ))
        .unwrap();
        let response = self.client.get(url).send().await?;
        response.error_for_status_ref()?;
        Ok(response.json().await?)
    }

    pub async fn get_category_by_slug<S>(&self, slug: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = Url::parse(&format!(
            "https://api.lu.ma/discover/category/get-page?slug={}",
            slug.as_ref()
        ))
        .unwrap();
        let response = self.client.get(url).send().await?;
        response.error_for_status_ref()?;
        Ok(response.json().await?)
    }

    pub async fn get_calendars_for_place<S>(&self, place: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = Url::parse(&format!(
            "https://api.lu.ma/discover/get-calendars?discover_place_api_id={}",
            place.as_ref()
        ))
        .unwrap();
        let response = self.client.get(url).send().await?;
        response.error_for_status_ref()?;
        Ok(response.json().await?)
    }

    pub async fn get_place_by_id<S>(&self, id: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = Url::parse(&format!(
            "https://api.lu.ma/discover/get-place-v2?discover_place_api_id={}",
            id.as_ref()
        ))
        .unwrap();
        let response = self.client.get(url).send().await?;
        response.error_for_status_ref()?;
        Ok(response.json().await?)
    }

    pub async fn get_place_by_slug<S>(&self, slug: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = Url::parse(&format!(
            "https://api.lu.ma/discover/get-place-v2?slug={}",
            slug.as_ref()
        ))
        .unwrap();
        let response = self.client.get(url).send().await?;
        response.error_for_status_ref()?;
        Ok(response.json().await?)
    }

    pub async fn get_place_events<S1, S2>(
        &self,
        place_id: S1,
        cursor: Option<S2>,
        limit: Option<u32>,
    ) -> Result<Pagination>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let url = Url::parse(&format!(
            "https://api.lu.ma/discover/get-paginated-events?discover_place_api_id={}{}{}",
            place_id.as_ref(),
            if let Some(cursor) = cursor {
                format!("&pagination_cursor={}", cursor.as_ref())
            } else {
                "".to_string()
            },
            if let Some(limit) = limit {
                format!("&pagination_limit={limit}")
            } else {
                "".to_string()
            }
        ))
        .unwrap();
        let response = self.client.get(url).send().await?;
        response.error_for_status_ref()?;
        Ok(response.json().await?)
    }

    pub async fn get_calendar_by_id<S>(&self, id: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = Url::parse(&format!(
            "https://api.lu.ma/calendar/get?api_id={}",
            id.as_ref()
        ))
        .unwrap();
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    pub async fn get_calendar_events<S1, S2>(
        &self,
        calendar: S1,
        cursor: Option<S2>,
        limit: Option<u32>,
    ) -> Result<serde_json::Value>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let url = Url::parse(&format!(
            "https://api.lu.ma/calendar/get-items?calendar_api_id={}{}{}&period=future",
            calendar.as_ref(),
            if let Some(cursor) = cursor {
                format!("&pagination_cursor={}", cursor.as_ref())
            } else {
                "".to_string()
            },
            if let Some(limit) = limit {
                format!("&pagination_limit={limit}")
            } else {
                "".to_string()
            }
        ))
        .unwrap();
        let response = self.client.get(url).send().await?;
        response.error_for_status_ref()?;
        Ok(response.json().await?)
    }

    pub async fn get_event<S>(&self, event: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = Url::parse(&format!(
            "https://api.lu.ma/event/get?event_api_id={}",
            event.as_ref()
        ))
        .unwrap();
        let response = self.client.get(url).send().await?;
        response.error_for_status_ref()?;
        Ok(response.json().await?)
    }

    pub async fn get_nearby_events<S>(&self, category: Option<S>) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = Url::parse(&format!(
            "https://api.lu.ma/discover/get-paginated-events?pagination_limit=50{}",
            if let Some(category) = category {
                format!("&slug={}", category.as_ref())
            } else {
                "".to_string()
            }
        ))
        .unwrap();
        let response = self.client.get(url).send().await?;
        response.error_for_status_ref()?;
        Ok(response.json().await?)
    }
}
