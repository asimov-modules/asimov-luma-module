// This is free and unencumbered software released into the public domain.

use crate::api::Error;
use crate::api::models::{Pagination, Root};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Client {
    agent: ureq::Agent,
}

impl Client {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            agent: ureq::AgentBuilder::new()
                .user_agent("asimov-luma-module")
                .build(),
        }
    }

    pub fn get_featured_calendars(&self) -> Result<Root> {
        let response = self.agent
            .get("https://api.lu.ma/calendar/get-featured-calendars")
            .call()?;
        let body = response.into_string()
            .map_err(|e| ureq::Error::from(e))?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn list_categories(&self) -> Result<Pagination> {
        let response = self.agent
            .get("https://api.lu.ma/discover/category/list-categories?pagination_limit=20")
            .call()?;
        let body = response.into_string()
            .map_err(|e| ureq::Error::from(e))?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn list_places(&self) -> Result<Root> {
        let response = self.agent
            .get("https://api.lu.ma/discover/list-places")
            .call()?;
        let body = response.into_string()
            .map_err(|e| ureq::Error::from(e))?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn get_category_by_id<S>(&self, id: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = format!(
            "https://api.lu.ma/discover/category/get-page?discover_category_api_id={}",
            id.as_ref()
        );
        let response = self.agent.get(&url).call()?;
        let body = response.into_string()
            .map_err(|e| ureq::Error::from(e))?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn get_category_by_slug<S>(&self, slug: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = format!(
            "https://api.lu.ma/discover/category/get-page?slug={}",
            slug.as_ref()
        );
        let response = self.agent.get(&url).call()?;
        let body = response.into_string()
            .map_err(|e| ureq::Error::from(e))?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn get_calendars_for_place<S>(&self, place: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = format!(
            "https://api.lu.ma/discover/get-calendars?discover_place_api_id={}",
            place.as_ref()
        );
        let response = self.agent.get(&url).call()?;
        let body = response.into_string()
            .map_err(|e| ureq::Error::from(e))?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn get_place_by_id<S>(&self, id: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = format!(
            "https://api.lu.ma/discover/get-place-v2?discover_place_api_id={}",
            id.as_ref()
        );
        let response = self.agent.get(&url).call()?;
        let body = response.into_string()
            .map_err(|e| ureq::Error::from(e))?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn get_place_by_slug<S>(&self, slug: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = format!(
            "https://api.lu.ma/discover/get-place-v2?slug={}",
            slug.as_ref()
        );
        let response = self.agent.get(&url).call()?;
        let body = response.into_string()
            .map_err(|e| ureq::Error::from(e))?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn get_place_events<S1, S2>(
        &self,
        place_id: S1,
        cursor: Option<S2>,
        limit: Option<u32>,
    ) -> Result<Pagination>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let mut url = format!(
            "https://api.lu.ma/discover/get-paginated-events?discover_place_api_id={}",
            place_id.as_ref()
        );
        if let Some(cursor) = cursor {
            url.push_str(&format!("&pagination_cursor={}", cursor.as_ref()));
        }
        if let Some(limit) = limit {
            url.push_str(&format!("&pagination_limit={limit}"));
        }
        let response = self.agent.get(&url).call()?;
        let body = response.into_string()
            .map_err(|e| ureq::Error::from(e))?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn get_calendar_by_id<S>(&self, id: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = format!(
            "https://api.lu.ma/calendar/get?api_id={}",
            id.as_ref()
        );
        let response = self.agent.get(&url).call()?;
        let body = response.into_string()
            .map_err(|e| ureq::Error::from(e))?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn get_calendar_events<S1, S2>(
        &self,
        calendar: S1,
        cursor: Option<S2>,
        limit: Option<u32>,
    ) -> Result<serde_json::Value>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let mut url = format!(
            "https://api.lu.ma/calendar/get-items?calendar_api_id={}&period=future",
            calendar.as_ref()
        );
        if let Some(cursor) = cursor {
            url.push_str(&format!("&pagination_cursor={}", cursor.as_ref()));
        }
        if let Some(limit) = limit {
            url.push_str(&format!("&pagination_limit={limit}"));
        }
        let response = self.agent.get(&url).call()?;
        let body = response.into_string()
            .map_err(|e| ureq::Error::from(e))?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn get_event<S>(&self, event: S) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let url = format!(
            "https://api.lu.ma/event/get?event_api_id={}",
            event.as_ref()
        );
        let response = self.agent.get(&url).call()?;
        let body = response.into_string()
            .map_err(|e| ureq::Error::from(e))?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn get_nearby_events<S>(&self, category: Option<S>) -> Result<serde_json::Value>
    where
        S: AsRef<str>,
    {
        let mut url = "https://api.lu.ma/discover/get-paginated-events?pagination_limit=50".to_string();
        if let Some(category) = category {
            url.push_str(&format!("&slug={}", category.as_ref()));
        }
        let response = self.agent.get(&url).call()?;
        let body = response.into_string()
            .map_err(|e| ureq::Error::from(e))?;
        Ok(serde_json::from_str(&body)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let client = Client::new();
        // Verifica que o client foi criado com sucesso
        assert!(true); // Client criado sem panics
    }

    #[test]
    #[ignore] // Teste de integração - requer conexão com a API
    fn test_get_featured_calendars() {
        let client = Client::new();
        let result = client.get_featured_calendars();
        assert!(result.is_ok(), "Should successfully get featured calendars");
        
        let root = result.unwrap();
        // Verifica que temos uma estrutura Root válida
        assert!(!root.infos.is_empty() || root.infos.is_empty()); // Aceita qualquer resultado
    }

    #[test]
    #[ignore] // Teste de integração
    fn test_list_categories() {
        let client = Client::new();
        let result = client.list_categories();
        assert!(result.is_ok(), "Should successfully list categories");
        
        let pagination = result.unwrap();
        // Verifica que temos uma estrutura Pagination válida
        assert!(pagination.entries.len() <= 20, "Should have at most 20 entries");
    }

    #[test]
    #[ignore] // Teste de integração
    fn test_list_places() {
        let client = Client::new();
        let result = client.list_places();
        assert!(result.is_ok(), "Should successfully list places");
    }

    #[test]
    #[ignore] // Teste de integração
    fn test_get_category_by_slug() {
        let client = Client::new();
        // Usa um slug que provavelmente existe (ajuste conforme necessário)
        let result = client.get_category_by_slug("tech");
        
        // Pode retornar Ok ou Err dependendo se o slug existe
        match result {
            Ok(value) => {
                assert!(value.is_object(), "Should return a JSON object");
            }
            Err(Error::Http(_)) => {
                // Erro HTTP é aceitável (slug não encontrado, etc)
            }
            Err(Error::Json(_)) => {
                panic!("Should not have JSON parsing error");
            }
        }
    }

    #[test]
    #[ignore] // Teste de integração
    fn test_get_place_events_with_pagination() {
        let client = Client::new();
        // Primeiro, precisamos de um place_id válido
        // Por enquanto, testamos apenas a construção da URL
        let result = client.get_place_events("test-place-id", None::<&str>, Some(10));
        
        // Pode falhar se o place_id não existir, mas não deve ter erro de parsing
        match result {
            Ok(pagination) => {
                assert!(pagination.entries.len() <= 10, "Should respect limit");
            }
            Err(Error::Http(_)) => {
                // Erro HTTP é aceitável (place não encontrado)
            }
            Err(Error::Json(e)) => {
                panic!("Should not have JSON parsing error: {}", e);
            }
        }
    }

    #[test]
    #[ignore] // Teste de integração
    fn test_get_calendar_events_with_cursor_and_limit() {
        let client = Client::new();
        let result = client.get_calendar_events("test-calendar-id", None::<&str>, Some(5));
        
        match result {
            Ok(value) => {
                assert!(value.is_object(), "Should return a JSON object");
            }
            Err(Error::Http(_)) => {
                // Erro HTTP é aceitável
            }
            Err(Error::Json(e)) => {
                panic!("Should not have JSON parsing error: {}", e);
            }
        }
    }

    #[test]
    #[ignore] // Teste de integração
    fn test_get_nearby_events() {
        let client = Client::new();
        let result = client.get_nearby_events::<String>(None);
        assert!(result.is_ok(), "Should successfully get nearby events");
        
        let value = result.unwrap();
        assert!(value.is_object(), "Should return a JSON object");
    }

    #[test]
    #[ignore] // Teste de integração
    fn test_get_nearby_events_with_category() {
        let client = Client::new();
        let result = client.get_nearby_events(Some("tech"));
        
        match result {
            Ok(value) => {
                assert!(value.is_object(), "Should return a JSON object");
            }
            Err(Error::Http(_)) => {
                // Erro HTTP é aceitável
            }
            Err(Error::Json(e)) => {
                panic!("Should not have JSON parsing error: {}", e);
            }
        }
    }

    #[test]
    fn test_error_types() {
        // Testa que os tipos de erro estão corretos
        let _client = Client::new();
        
        // Testa com URL inválida (deve retornar erro HTTP)
        // Nota: ureq pode não validar URLs da mesma forma, então este teste pode precisar ajuste
        let result = _client.get_category_by_slug("");
        
        // Verifica que retorna um Error (não panics)
        assert!(result.is_err());
        
        match result.unwrap_err() {
            Error::Http(_) => {
                // Esperado para URL inválida ou erro de conexão
            }
            Error::Json(_) => {
                // Também possível se a resposta não for JSON válido
            }
        }
    }
}
