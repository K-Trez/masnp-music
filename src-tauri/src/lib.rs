use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub url: String,
    pub duration: Option<u32>,
    pub thumbnail: Option<String>,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggregatedResult {
    pub results: Vec<SearchResult>,
    pub total_found: usize,
    pub latency_ms: u64,
}

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
pub enum MusicError {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Parsing error: {0}")]
    ParsingError(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

pub struct MusicService;

impl MusicService {
    pub async fn search(request: SearchRequest) -> Result<AggregatedResult, MusicError> {
        let query = request.query.clone();
        let results = vec![
            SearchResult {
                id: "1".to_string(),
                title: format!("Mock Song 1 for {}", query),
                artist: "Mock Artist".to_string(),
                url: "https://example.com/audio1.mp3".to_string(),
                duration: Some(180),
                thumbnail: Some("https://example.com/thumb1.jpg".to_string()),
                source: "Internet Archive".to_string(),
            },
            SearchResult {
                id: "2".to_string(),
                title: format!("Mock Song 2 for {}", query),
                artist: "Mock Artist 2".to_string(),
                url: "https://example.com/audio2.mp3".to_string(),
                duration: Some(210),
                thumbnail: Some("https://example.com/thumb2.jpg".to_string()),
                source: "Free Sounds".to_string(),
            },
        ];

        Ok(AggregatedResult {
            total_found: results.len(),
            latency_ms: 150,
            results,
        })
    }

    pub async fn get_app_info() -> Result<serde_json::Value, MusicError> {
        Ok(serde_json::json!({
            "name": "masnp-music",
            "version": "0.1.0",
            "status": "Development"
        }))
    }
}
