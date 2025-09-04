use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct TestResponse {
    #[allow(unused)]
    status: String,
    message: String,
}

#[component]
pub fn AboutPage() -> impl IntoView {
    let (data, set_data) = signal(None::<TestResponse>);

    Effect::new(move |_| {
        spawn_local(async move {
            match reqwasm::http::Request::get("/api/test").send().await {
                Ok(resp) => {
                    if let Ok(json) = resp.json::<TestResponse>().await {
                        leptos::logging::log!("JSON parsed successfully: {:?}", json);
                        set_data.set(Some(json));
                    } else {
                        leptos::logging::error!("Failed to parse JSON");
                        set_data.set(None);
                    }
                }
                Err(err) => {
                    leptos::logging::error!("Failed to fetch data: {}", err);
                    set_data.set(None);
                }
            }
        });
    });

    view! {
        <div class="p-4 h-screen dark:bg-zinc-800 dark:text-white">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-bold mb-4">"About Page"</h1>
                <A href="/b">
                    <button class="underline px-4 py-2 rounded-xl">
                        "Home"
                    </button>
                </A>
            </div>
            <div>
                {move || match data.get() {
                    Some(data) => view! { <div class="p-4">{data.message}</div> },
                    None => view! { <div class="p-4">{"No data available.".to_string()}</div> },
                }}
            </div>

            <pre>{move || format!("{:?}", data.get().unwrap_or_else(|| TestResponse { status: "".to_string(), message: "".to_string() } ))}</pre>
        </div>
    }
}
