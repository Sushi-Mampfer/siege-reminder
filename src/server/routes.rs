#[cfg(feature = "ssr")]
use crate::server::handlers;
#[cfg(feature = "ssr")]
use axum::{routing::get, Router};
#[cfg(feature = "ssr")]
use leptos_config::LeptosOptions;

#[cfg(feature = "ssr")]
pub fn api_routes() -> Router<LeptosOptions> {
    Router::new()
        .route("/api/test", get(handlers::test_handler))
        .with_state(())
}
