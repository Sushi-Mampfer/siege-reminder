use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::pages::HomePage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <link rel="stylesheet" id="leptos" href="/pkg/siege-reminder.css" />
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
                <MetaTags />
                <style>{r#"
                    input[type=number]::-webkit-inner-spin-button,
                    input[type=number]::-webkit-outer-spin-button {
                        -webkit-appearance: none;
                        margin: 0;
                    }

                    input[type=number] {
                        -moz-appearance: textfield;
                    }
                "#}</style>
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Welcome to Leptos" />
        <Router>
            <main class="h-screen w-screen grid grid-cols-3 bg-zinc-800 text-zinc-200">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}
