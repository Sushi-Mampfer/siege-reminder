#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::env;

    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use siege_reminder::{app::*, db::prep_db, notifications::notifications};
    use tokio::spawn;

    prep_db().await;
    dbg!("spawning");
    spawn(notifications());
    dbg!("spawned");

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    let options_for_routes = leptos_options.clone();

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, move || {
            shell(options_for_routes.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", env::var("PORT").unwrap_or("8080".to_string()))).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
