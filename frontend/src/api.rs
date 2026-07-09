use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use shared_core::types::{
    PinRequiredResponse, SiteConfig, TodoLists, VerifyPinRequest, VerifyPinResponse,
};

/// Wire envelope matching the backend [`TodoState`] / `data/todos.json` shape.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TodoEnvelope {
    #[serde(default)]
    pub version: u64,
    #[serde(default)]
    pub lists: TodoLists,
}

// Fetches the site configuration properties from the server
pub async fn fetch_config() -> Result<SiteConfig, gloo_net::Error> {
    Request::get("/api/config").send().await?.json().await
}

// Queries the PIN configuration requirements and IP lock status details
pub async fn fetch_pin_required() -> Result<PinRequiredResponse, gloo_net::Error> {
    Request::get("/api/pin-required").send().await?.json().await
}

// Fetches the raw HTTP Response for list todos (to check for 401 unauthorized status)
pub async fn fetch_todos_raw() -> Result<Response, gloo_net::Error> {
    Request::get("/api/todos").send().await
}

// Submits the user input digits to verify cookie authentication
pub async fn verify_pin(pin: &str) -> Result<VerifyPinResponse, gloo_net::Error> {
    Request::post("/api/verify-pin")
        .json(&VerifyPinRequest {
            pin: pin.to_string(),
        })?
        .send()
        .await?
        .json()
        .await
}

/// Saves lists with the optimistic-concurrency version the client last observed.
pub async fn save_todos(todos: &TodoLists, version: u64) -> Result<Response, gloo_net::Error> {
    let body = TodoEnvelope {
        version,
        lists: todos.clone(),
    };
    Request::post("/api/todos").json(&body)?.send().await
}

// Submits a POST request to logout and clear authentication cookies
pub async fn logout() -> Result<bool, gloo_net::Error> {
    let resp = Request::post("/api/logout").send().await?;
    Ok(resp.ok())
}
