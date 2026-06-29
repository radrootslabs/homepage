use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

#[derive(Clone, Debug, Serialize)]
pub struct SupportContactValues {
    pub display_name: String,
    pub outreach_method: String,
    pub contact_address: String,
    pub message: String,
}

#[derive(Clone, Debug)]
pub enum SupportContactSubmitResult {
    Accepted,
    Duplicate,
    Rejected,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SupportContactErrorCode {
    DuplicateDedupeClaim,
    DuplicateRequest,
    SupportApiUnavailable,
    Unknown,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: Option<ErrorBody>,
}

#[derive(Deserialize)]
struct ErrorBody {
    code: Option<String>,
}

pub async fn submit_support_contact(
    endpoint: &str,
    values: SupportContactValues,
    locale: String,
) -> SupportContactSubmitResult {
    let request_id = format!("homepage-contact-{}", js_sys::Date::now() as u64);
    let payload = build_support_contact_payload(&values, &request_id, &locale);
    let request = match Request::post(endpoint)
        .header("accept", "application/json")
        .json(&payload)
    {
        Ok(request) => request,
        Err(_) => return SupportContactSubmitResult::Rejected,
    };
    let response = match request.send().await {
        Ok(response) => response,
        Err(_) => return SupportContactSubmitResult::Unavailable,
    };

    if response.ok() {
        return SupportContactSubmitResult::Accepted;
    }

    match response_error_code(response).await {
        SupportContactErrorCode::DuplicateDedupeClaim
        | SupportContactErrorCode::DuplicateRequest => SupportContactSubmitResult::Duplicate,
        SupportContactErrorCode::SupportApiUnavailable => SupportContactSubmitResult::Unavailable,
        SupportContactErrorCode::Unknown => SupportContactSubmitResult::Rejected,
    }
}

fn build_support_contact_payload(
    values: &SupportContactValues,
    request_id: &str,
    locale: &str,
) -> Value {
    let mut identity = Map::new();
    insert_optional_text(&mut identity, "display_name", &values.display_name);
    insert_optional_text(&mut identity, "locale", locale);

    match values.outreach_method.as_str() {
        "email" => insert_optional_text(&mut identity, "email", &values.contact_address),
        "nostr" => insert_optional_text(&mut identity, "nostr", &values.contact_address),
        _ => {}
    }

    json!({
        "request_id": request_id,
        "contact_us_type": values.outreach_method,
        "outreach_method": values.outreach_method,
        "identity": Value::Object(identity),
        "message": values.message.trim(),
    })
}

async fn response_error_code(response: gloo_net::http::Response) -> SupportContactErrorCode {
    response
        .json::<ErrorResponse>()
        .await
        .ok()
        .and_then(|body| body.error)
        .and_then(|error| error.code)
        .map(|code| match code.as_str() {
            "duplicate_dedupe_claim" => SupportContactErrorCode::DuplicateDedupeClaim,
            "duplicate_request" => SupportContactErrorCode::DuplicateRequest,
            "support_api_unavailable" => SupportContactErrorCode::SupportApiUnavailable,
            _ => SupportContactErrorCode::Unknown,
        })
        .unwrap_or(SupportContactErrorCode::Unknown)
}

fn insert_optional_text(map: &mut Map<String, Value>, key: &str, value: &str) {
    let trimmed = value.trim();
    if !trimmed.is_empty() {
        map.insert(key.to_owned(), Value::String(trimmed.to_owned()));
    }
}
