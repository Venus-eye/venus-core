use axum::{routing::post, Json, Router};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::providers::whoisxml::WhoisXmlProvider;

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

fn email_regex() -> &'static Regex {
    EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$").unwrap()
    })
}

fn http_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(Client::new)
}

fn parse_bool(s: &Option<String>) -> Option<bool> {
    s.as_deref().map(|v| v == "true")
}

#[derive(Deserialize)]
struct EmailRequest {
    email: String,
}

#[derive(Serialize)]
struct EmailResponse {
    email: String,
    format_valid: bool,
    smtp_valid: Option<bool>,
    dns_valid: Option<bool>,
    disposable: Option<bool>,
    free_provider: Option<bool>,
    mx_records: Option<Vec<String>>,
    error: Option<String>,
}

async fn validate_email(Json(payload): Json<EmailRequest>) -> Json<EmailResponse> {
    let format_valid = email_regex().is_match(&payload.email);

    if !format_valid {
        return Json(EmailResponse {
            email: payload.email,
            format_valid: false,
            smtp_valid: None,
            dns_valid: None,
            disposable: None,
            free_provider: None,
            mx_records: None,
            error: None,
        });
    }

    match WhoisXmlProvider.verify(http_client(), &payload.email).await {
        Err(e) => Json(EmailResponse {
            email: payload.email,
            format_valid: true,
            smtp_valid: None,
            dns_valid: None,
            disposable: None,
            free_provider: None,
            mx_records: None,
            error: Some(e.to_string()),
        }),
        Ok(data) => Json(EmailResponse {
            email: payload.email,
            format_valid: true,
            smtp_valid: parse_bool(&data.smtp_check),
            dns_valid: parse_bool(&data.dns_check),
            disposable: parse_bool(&data.disposable_check),
            free_provider: parse_bool(&data.free_check),
            mx_records: data.mx_records,
            error: None,
        }),
    }
}

pub fn create_router() -> Router {
    Router::new().route("/api/validate-email", post(validate_email))
}
