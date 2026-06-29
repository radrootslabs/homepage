use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

#[derive(Clone, Debug, Serialize)]
pub struct ContactValues {
    pub display_name: String,
    pub outreach_method: String,
    pub contact_address: String,
    pub message: String,
}

#[derive(Clone, Debug)]
pub enum ContactSubmitResult {
    Accepted,
    Duplicate,
    Rejected(ContactRejection),
    Unavailable,
}

#[derive(Clone, Debug)]
pub struct ContactRejection {
    pub code: String,
    pub field: Option<String>,
    pub message: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ContactErrorCode {
    DuplicateDedupeClaim,
    DuplicateRequest,
    ContactApiUnavailable,
    Unknown,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: Option<ErrorBody>,
}

#[derive(Deserialize)]
struct ErrorBody {
    code: Option<String>,
    field: Option<String>,
    message: Option<String>,
}

pub async fn submit_contact(
    endpoint: &str,
    values: ContactValues,
    locale: String,
) -> ContactSubmitResult {
    let request_id = format!("homepage-contact-{}", js_sys::Date::now() as u64);
    let payload = build_contact_payload(&values, &request_id, &locale);
    let request = match Request::post(endpoint)
        .header("accept", "application/json")
        .json(&payload)
    {
        Ok(request) => request,
        Err(_) => return ContactSubmitResult::Rejected(ContactRejection::request_error()),
    };
    let response = match request.send().await {
        Ok(response) => response,
        Err(_) => return ContactSubmitResult::Unavailable,
    };

    if response.ok() {
        return ContactSubmitResult::Accepted;
    }

    let rejection = response_rejection(response).await;
    match ContactErrorCode::from_code(&rejection.code) {
        ContactErrorCode::DuplicateDedupeClaim | ContactErrorCode::DuplicateRequest => {
            ContactSubmitResult::Duplicate
        }
        ContactErrorCode::ContactApiUnavailable => ContactSubmitResult::Unavailable,
        ContactErrorCode::Unknown => ContactSubmitResult::Rejected(rejection),
    }
}

fn build_contact_payload(values: &ContactValues, request_id: &str, locale: &str) -> Value {
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

async fn response_rejection(response: gloo_net::http::Response) -> ContactRejection {
    response
        .json::<ErrorResponse>()
        .await
        .ok()
        .and_then(|body| body.error)
        .map(|error| ContactRejection {
            code: error.code.unwrap_or_else(|| "unknown_error".to_owned()),
            field: error.field,
            message: error.message,
        })
        .unwrap_or_else(ContactRejection::unknown)
}

fn insert_optional_text(map: &mut Map<String, Value>, key: &str, value: &str) {
    let trimmed = value.trim();
    if !trimmed.is_empty() {
        map.insert(key.to_owned(), Value::String(trimmed.to_owned()));
    }
}

impl ContactErrorCode {
    fn from_code(code: &str) -> Self {
        match code {
            "duplicate_dedupe_claim" => Self::DuplicateDedupeClaim,
            "duplicate_request" => Self::DuplicateRequest,
            "contact_api_unavailable" => Self::ContactApiUnavailable,
            _ => Self::Unknown,
        }
    }
}

impl ContactRejection {
    fn request_error() -> Self {
        Self {
            code: "invalid_request".to_owned(),
            field: None,
            message: None,
        }
    }

    fn unknown() -> Self {
        Self {
            code: "unknown_error".to_owned(),
            field: None,
            message: None,
        }
    }
}
