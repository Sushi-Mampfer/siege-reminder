#[cfg(feature = "ssr")]
pub mod handlers;
#[cfg(feature = "ssr")]
pub mod routes;

#[cfg(feature = "ssr")]
use axum::Router;
#[cfg(feature = "ssr")]
use leptos_config::LeptosOptions;

#[cfg(feature = "ssr")]
pub fn create_server_routes(_leptos_options: &LeptosOptions) -> Router<LeptosOptions> {
    Router::new().merge(routes::api_routes())
}
