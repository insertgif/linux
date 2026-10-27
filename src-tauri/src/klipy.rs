use serde::Deserialize;

use crate::provider::Gif;

const BASE_URL: &str = "https://api.klipy.com/api/v1";
const CUSTOMER_ID: &str = "insertgif";

pub struct KlipyProvider {
    api_key: String,
    client: reqwest::Client,
}

#[derive(Deserialize)]
struct Response {
    result: bool,
    data: Option<Page>,
    message: Option<String>,
}

#[derive(Deserialize)]
struct Page {
    data: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
    id: serde_json::Value,
    #[serde(default)]
    title: String,
    file: Files,
}

#[derive(Deserialize)]
struct Files {
    hd: Option<Formats>,
    md: Option<Formats>,
    sm: Option<Formats>,
}

#[derive(Deserialize)]
struct Formats {
    gif: Option<Media>,
}

#[derive(Deserialize)]
struct Media {
    url: String,
}

impl KlipyProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key, client: reqwest::Client::new() }
    }

    pub async fn search(&self, query: &str, limit: u32) -> Result<Vec<Gif>, String> {
        self.fetch("search", &[("q", query), ("per_page", &limit.to_string())]).await
    }

    pub async fn trending(&self, limit: u32) -> Result<Vec<Gif>, String> {
        self.fetch("trending", &[("per_page", &limit.to_string())]).await
    }

    async fn fetch(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<Vec<Gif>, String> {
        if self.api_key.is_empty() {
            return Err("No KLIPY API key configured (see README)".into());
        }

        let url = format!("{BASE_URL}/{}/gifs/{endpoint}", self.api_key);
        let response = self
            .client
            .get(url)
            .query(params)
            .query(&[("customer_id", CUSTOMER_ID)])
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e.without_url()))?;

        let status = response.status();
        let body: Response = response
            .json()
            .await
            .map_err(|e| format!("Bad response from KLIPY (HTTP {status}): {}", e.without_url()))?;

        if !status.is_success() || !body.result {
            let message = body.message.unwrap_or_else(|| format!("HTTP {status}"));
            return Err(format!("KLIPY error: {message}"));
        }

        Ok(body
            .data
            .map(|page| page.data)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|item| {
                let url = [item.file.hd, item.file.md, item.file.sm]
                    .into_iter()
                    .flatten()
                    .find_map(|formats| formats.gif)?
                    .url;
                let id = match item.id {
                    serde_json::Value::String(s) => s,
                    other => other.to_string(),
                };
                Some(Gif { id, title: item.title, url })
            })
            .collect())
    }
}
