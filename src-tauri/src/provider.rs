use serde::Serialize;

use crate::klipy::KlipyProvider;

#[derive(Debug, Clone, Serialize)]
pub struct Gif {
    pub id: String,
    pub title: String,
    pub url: String,
}

/// The GIF backends the app can talk to. An enum rather than a trait object
/// keeps the async methods simple.
pub enum AnyProvider {
    Klipy(KlipyProvider),
}

impl AnyProvider {
    pub async fn search(&self, query: &str, limit: u32) -> Result<Vec<Gif>, String> {
        match self {
            AnyProvider::Klipy(p) => p.search(query, limit).await,
        }
    }

    pub async fn trending(&self, limit: u32) -> Result<Vec<Gif>, String> {
        match self {
            AnyProvider::Klipy(p) => p.trending(limit).await,
        }
    }
}
