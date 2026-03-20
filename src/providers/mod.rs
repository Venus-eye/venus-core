use async_trait::async_trait;
use crate::models::CheckResult;
use reqwest::Client;

pub mod whoisxml;

#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    async fn check(&self, client: &Client, target: &str) -> CheckResult;
}