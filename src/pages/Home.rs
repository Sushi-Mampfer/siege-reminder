use chrono::{Datelike, Duration, Local, NaiveTime, Offset, TimeZone, Utc};
use gloo_timers::callback::Interval;
use leptos::{
    ev::SubmitEvent, logging::log, prelude::*, task::spawn_local
};

use crate::{datatypes::Settings, query_data, set_project, set_times};

#[component]
pub fn HomePage() -> impl IntoView {
    let (time, set_time) = signal(Local::now().format("%H:%M:%S").to_string());
    let (username, set_username) = signal("".to_string());
    let (primary, set_primary) = signal("".to_string());
    let monday = RwSignal::new("18:00".to_string());
    let tuesday = RwSignal::new("18:00".to_string());
    let wednesday = RwSignal::new("18:00".to_string());
    let thursday = RwSignal::new("18:00".to_string());
    let friday = RwSignal::new("18:00".to_string());
    let saturday = RwSignal::new("18:00".to_string());
    let sunday = RwSignal::new("18:00".to_string());

    let project_loader = Resource::new(move || username.get(), |username| query_data(username));

    let update = move |ev: SubmitEvent| {
        ev.prevent_default();
        let settings = Settings {
            monday: monday.get(),
            tuesday: tuesday.get(),
            wednesday: wednesday.get(),
            thursday: thursday.get(),
            friday: friday.get(),
            saturday: saturday.get(),
            sunday: sunday.get(),
        };
        let username = username.get();
        spawn_local(async move { set_times(username, settings).await; });
    };

    let load_settings = move |settings: Settings| {
        monday.set(settings.monday);
        tuesday.set(settings.tuesday);
        wednesday.set(settings.wednesday);
        thursday.set(settings.thursday);
        friday.set(settings.friday);
        saturday.set(settings.saturday);
        sunday.set(settings.sunday);
    };

    #[cfg(target_arch = "wasm32")]
    Interval::new(1_000, move || {
        set_time.set(Local::now().format("%H:%M:%S").to_string());
    }).forget();

    let next_sunday_local = NaiveTime::from_hms_opt(5, 0, 0).unwrap().overflowing_sub_signed(Duration::seconds(Local::now().offset().fix().utc_minus_local() as i64)).0.format("%H:%M").to_string();

    view! {
        <div>
            <form on:submit=move |ev| {
                ev.prevent_default();
                project_loader.refetch();
            }>
                <label for="username">Username:</label>
                <input
                    id="username"
                    name="username"
                    placeholder="Username"
                    bind:value=(username, set_username)
                    on:input:target=move |ev| { set_username.set(ev.target().value()) }
                />
                <input type="submit" value="Load settings" />
            </form>
            <form on:submit=update>
                <ul>
                    <li>Monday <input name="monday" type="time" bind:value=monday /></li>
                    <li>Tuesday <input name="tuesday" type="time" bind:value=tuesday /></li>
                    <li>Wednesday <input name="wednesday" type="time" bind:value=wednesday /></li>
                    <li>Thursday <input name="thursday" type="time" bind:value=thursday /></li>
                    <li>Friday <input name="friday" type="time" bind:value=friday /></li>
                    <li>Saturday <input name="saturday" type="time" bind:value=saturday /></li>
                    <li>
                        Sunday 
                        <input name="sunday" type="time" bind:value=sunday />
                    </li>
                </ul>
                <input type="submit" value="Update" />
            </form>
        </div>
        <div><p>Your current time is {time}</p><p>If not please adjust the times accordingly.</p><p>You 'll have to submit at {next_sunday_local}</p></div>
        <div>
            <Suspense fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || {
                    project_loader
                        .get()
                        .map(|res| {
                            match res {
                                Ok(data) => {
                                    set_primary.set(data.primary);
                                    load_settings(data.settings);
                                    view! {
                                        <ul>
                                            {
                                                data.projects
                                                    .into_iter()
                                                    .map(|p| {
                                                        let name_check = p.name.clone();
                                                        view! {
                                                            <li
                                                                class:bg-red-900=move || {
                                                                    primary.get() == name_check
                                                                }
                                                                on:click=move |_| {
                                                                    let project = p.name.clone();
                                                                    let username = username.get();
                                                                    set_primary.set(p.name.clone());
                                                                    spawn_local(async move {
                                                                        set_project(username, project).await;
                                                                    });
                                                                }
                                                            >
                                                                <p>{p.name.clone()}</p>
                                                                <p>{p.time}</p>
                                                            </li>
                                                        }
                                                    })
                                                    .collect_view()
                                            }
                                        </ul>
                                    }
                                        .into_any()
                                }
                                Err(e) => {
                                    match e {
                                        ServerFnError::ServerError(e) => {
                                            view! { <p>{e}</p> }.into_any()
                                        }
                                        e => view! { <p>{e.to_string()}</p> }.into_any(),
                                    }
                                }
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}
