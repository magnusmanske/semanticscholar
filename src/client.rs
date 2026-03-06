use crate::error::Error;
use crate::work::Work;

const API_BASE_URL: &str = "https://api.semanticscholar.org/v1";

#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
}

impl Client {
    #[must_use]
    pub fn new() -> Client {
        Client {
            http: reqwest::Client::new(),
        }
    }

    /// Fetches a work (paper) by its identifier (DOI, arxiv ID, etc.).
    ///
    /// # Errors
    ///
    /// Returns `Error::Http` if the HTTP request fails, or `Error::Api` if the
    /// API returns an error response, or `Error::InvalidJson` if the response
    /// cannot be parsed.
    pub async fn work(&self, id: &str) -> Result<Work, Error> {
        let url = format!("{API_BASE_URL}/paper/{id}");
        let json: serde_json::Value = self.http.get(&url).send().await?.json().await?;
        match json["error"].as_str() {
            Some(error_string) => Err(Error::Api(format!("{error_string}:{id}"))),
            None => Work::new_from_json(&json),
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let client = Client::new();
        let debug = format!("{:?}", client);
        assert!(debug.contains("Client"));
    }

    #[test]
    fn test_client_default() {
        let client = Client::default();
        let debug = format!("{:?}", client);
        assert!(debug.contains("Client"));
    }

    #[test]
    fn test_client_clone() {
        let client = Client::new();
        let _cloned = client.clone();
    }
}
