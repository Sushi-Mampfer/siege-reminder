#[cfg(feature = "ssr")]
use axum::Json;
use serde_json::json;

#[cfg(feature = "ssr")]
pub async fn test_handler() -> Json<serde_json::Value> {
    leptos::logging::log!("Test endpoint working!");
    Json(json!({
        "status": "ok",
        "message": "Test endpoint working!"
    }))
}
