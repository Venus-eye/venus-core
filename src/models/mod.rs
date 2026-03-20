use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckResult {
    pub provider_name: String,
    pub target: String,
    pub exists: bool,
    pub profile_url: Option<String>,
    pub confidence_score: f32,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub target: String,
}