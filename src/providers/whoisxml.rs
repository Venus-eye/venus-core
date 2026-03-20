use reqwest::Client;
use serde::Deserialize;

const API_URL: &str = "https://emailverification.whoisxmlapi.com/api/v3";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhoisXmlResponse {
    pub smtp_check: Option<String>,
    pub dns_check: Option<String>,
    pub disposable_check: Option<String>,
    pub free_check: Option<String>,
    pub mx_records: Option<Vec<String>>,
}

pub struct WhoisXmlProvider;

impl WhoisXmlProvider {
    pub async fn verify(&self, client: &Client, email: &str) -> Result<WhoisXmlResponse, reqwest::Error> {
        let api_key = std::env::var("WHOISXML_API_KEY")
            .expect("WHOISXML_API_KEY must be set in .env");

        client
            .get(API_URL)
            .query(&[("apiKey", api_key.as_str()), ("emailAddress", email)])
            .send()
            .await?
            .json::<WhoisXmlResponse>()
            .await
    }
}
