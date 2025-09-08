use chrono::{Datelike, Days, Duration, Local, NaiveTime, Offset, Utc};
use gloo_timers::callback::Interval;
use leptos::{ev::SubmitEvent, logging::log, on_mount, prelude::*, task::spawn_local};

use crate::{datatypes::Settings, query_data, set_project, set_times};

#[component]
pub fn HomePage() -> impl IntoView {
    let offset = Local::now().offset().fix();
    let monday_date = Utc::now()
        .date_naive()
        .checked_sub_signed(Duration::days(
            Utc::now().weekday().num_days_from_monday() as i64
        ))
        .unwrap();

    let to_utc =
        move |orig_time: String, days: u64| match NaiveTime::parse_from_str(&orig_time, "%H:%M") {
            Ok(t) => monday_date
                .and_time(t)
                .checked_add_days(Days::new(days))
                .unwrap()
                .checked_sub_offset(offset)
                .unwrap()
                .signed_duration_since(monday_date.and_hms_opt(0, 0, 0).unwrap())
                .num_minutes(),
            Err(_) => 0,
        };

    let from_utc = move |orig_time: i64| {
        monday_date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .checked_add_signed(Duration::minutes(orig_time))
            .unwrap()
            .checked_add_offset(offset)
            .unwrap()
            .format("%H:%M")
            .to_string()
    };

    let (next_sunday_local, set_next_sunday_local) = signal("04:00".to_string());
    let (time, set_time) = signal(Local::now().format("%H:%M:%S").to_string());
    let (username, set_username) = signal("".to_string());
    let (primary, set_primary) = signal("".to_string());

    let monday = RwSignal::new("18:00".to_string());
    let monday_goal = RwSignal::new("1".to_string());
    let tuesday = RwSignal::new("18:00".to_string());
    let tuesday_goal = RwSignal::new("1".to_string());
    let wednesday = RwSignal::new("18:00".to_string());
    let wednesday_goal = RwSignal::new("1".to_string());
    let thursday = RwSignal::new("18:00".to_string());
    let thursday_goal = RwSignal::new("1".to_string());
    let friday = RwSignal::new("18:00".to_string());
    let friday_goal = RwSignal::new("1".to_string());
    let saturday = RwSignal::new("18:00".to_string());
    let saturday_goal = RwSignal::new("3".to_string());
    let sunday = RwSignal::new("18:00".to_string());
    let sunday_goal = RwSignal::new("3".to_string());

    let project_loader = Resource::new(move || username.get(), |username| query_data(username));

    let update = move |ev: SubmitEvent| {
        ev.prevent_default();
        let settings = Settings {
            monday: (
                to_utc(monday.get(), 0),
                monday_goal.get().parse().expect("Leave the inputs"),
            ),
            tuesday: (
                to_utc(tuesday.get(), 1),
                tuesday_goal.get().parse().expect("Leave the inputs"),
            ),
            wednesday: (
                to_utc(wednesday.get(), 2),
                wednesday_goal.get().parse().expect("Leave the inputs"),
            ),
            thursday: (
                to_utc(thursday.get(), 3),
                thursday_goal.get().parse().expect("Leave the inputs"),
            ),
            friday: (
                to_utc(friday.get(), 4),
                friday_goal.get().parse().expect("Leave the inputs"),
            ),
            saturday: (
                to_utc(saturday.get(), 5),
                saturday_goal.get().parse().expect("Leave the inputs"),
            ),
            sunday: (
                to_utc(sunday.get(), 6),
                sunday_goal.get().parse().expect("Leave the inputs"),
            ),
        };

        let username = username.get();
        spawn_local(async move {
            set_times(username, settings).await;
        });
    };

    let load_settings = move |settings: Settings| {
        monday.set(from_utc(settings.monday.0));
        monday_goal.set(settings.monday.1.to_string());
        tuesday.set(from_utc(settings.tuesday.0));
        tuesday_goal.set(settings.tuesday.1.to_string());
        wednesday.set(from_utc(settings.wednesday.0));
        wednesday_goal.set(settings.wednesday.1.to_string());
        thursday.set(from_utc(settings.thursday.0));
        thursday_goal.set(settings.thursday.1.to_string());
        friday.set(from_utc(settings.friday.0));
        friday_goal.set(settings.friday.1.to_string());
        saturday.set(from_utc(settings.saturday.0));
        saturday_goal.set(settings.saturday.1.to_string());
        sunday.set(from_utc(settings.sunday.0));
        sunday_goal.set(settings.sunday.1.to_string());
    };

    Effect::new(move || {
        if let Some(res) = project_loader.get() {
            match res {
                Ok(d) => {
                    set_primary.set(d.primary);
                    load_settings(d.settings);
                }
                Err(_) => (),
            }
        }
    });

    #[cfg(target_arch = "wasm32")]
    Interval::new(1_000, move || {
        set_time.set(Local::now().format("%H:%M:%S").to_string());
    })
    .forget();

    on_mount(move || {
        let now = chrono::Local::now();
        let time = now
            .date_naive()
            .and_hms_opt(4, 0, 0)
            .unwrap()
            .checked_add_days(chrono::Days::new(
                7 - now.weekday().num_days_from_sunday() as u64,
            ))
            .unwrap()
            .format("%H:%M")
            .to_string();

        set_next_sunday_local.set(time);
    });

    view! {
        <div class="col-start-1 row-start-1 justify-self-center pt-5">
            <form on:submit=move |ev| {
                ev.prevent_default();
                project_loader.refetch();
            }>
                <input
                    class="bg-zinc-700 p-1 h-12 leading-12 w-full rounded-[3rem] text-center m-1 focus:outline-none"
                    id="username"
                    name="username"
                    placeholder="Username"
                    bind:value=(username, set_username)
                    on:input:target=move |ev| { set_username.set(ev.target().value()) }
                />
                <br />
                <input class="bg-zinc-700 h-12 w-full rounded-[3rem] text-center m-1 flex items-center justify-center hover:bg-zinc-600" type="submit" value="Load settings" />
            </form>
            <form on:submit=update>
                <div class="grid grid-cols-[repeat(4,25%)]">
                    <p class="col-start-1 h-12 leading-12">Monday:</p><input class="col-start-2 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem]" name="monday" type="time" bind:value=monday /> <p class="col-start-3 h-12 leading-12 text-center">Goal:</p> <input class="col-start-4 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem]" name="monday_goal" type="number" bind:value=monday_goal />
                    <p class="col-start-1 h-12 leading-12">Tuesday:</p><input class="fcol-start-2 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem]" name="tuesday" type="time" bind:value=tuesday /> <p class="col-start-3 h-12 leading-12 text-center">Goal:</p> <input class="col-start-4 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem]" name="tuesday_goal" type="number" bind:value=tuesday_goal />
                    <p class="col-start-1 h-12 leading-12">Wednesday:</p><input class="fcol-start-2 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem]" name="wednesday" type="time" bind:value=wednesday /> <p class="col-start-3 h-12 leading-12 text-center">Goal:</p> <input class="col-start-4 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem]" name="wednesday_goal" type="number" bind:value=wednesday_goal />
                    <p class="col-start-1 h-12 leading-12">Thursday:</p><input class="fcol-start-2 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem]" name="thursday" type="time" bind:value=thursday /> <p class="col-start-3 h-12 leading-12 text-center">Goal:</p> <input class="col-start-4 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem]" name="thursday_goal" type="number" bind:value=thursday_goal />
                    <p class="col-start-1 h-12 leading-12">Friday:</p><input class="fcol-start-2 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem]" name="friday" type="time" bind:value=friday /> <p class="col-start-3 h-12 leading-12 text-center">Goal:</p> <input class="col-start-4 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem]" name="friday_goal" type="number" bind:value=friday_goal />
                    <p class="col-start-1 h-12 leading-12">Saturday:</p><input class="fcol-start-2 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem]" name="saturday" type="time" bind:value=saturday /> <p class="col-start-3 h-12 leading-12 text-center">Goal:</p> <input class="col-start-4 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem]" name="saturday_goal" type="number" bind:value=saturday_goal />
                    <p class="col-start-1 h-12 leading-12">Sunday:</p><input class="col-start-2 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem] mb-4" name="sunday" type="time" bind:value=sunday /> <p class="col-start-3 h-12 leading-12 text-center">Goal:</p> <input class="col-start-4 h-[calc(3rem-.5rem)] leading-[calc(3rem-.5rem)] bg-zinc-700 text-center rounded-[calc(3rem-.5rem)] focus:outline-none m-[0.25rem]" name="sunday_goal" type="number" bind:value=sunday_goal />
                </div>
                <input class="bg-zinc-700 text-center h12 leading-12 w-full rounded-[3rem] hover:bg-zinc-600" type="submit" value="Save" />
            </form>
        </div>
        <div class="col-start-2 row-start-1 justify-self-center">
            <div>
                <p class="text-center pt-5">Your current time is {time}</p>
                <p class="text-center pt-2">If not please adjust the times accordingly.</p>
                <p class="text-center pt-2">{format!("You'll have to submit at {}", next_sunday_local)}</p>
            </div>
            <div>
                <h1 class="pt-5 text-[5rem] text-center font-bold">Tutorial</h1>
                <ul>
                    <li class="text-center leading-7">Put in your hackatime username</li>
                    <li class="text-center leading-7">Input your desired notification times and goals</li>
                    <li class="text-center leading-7">Select your project</li>
                    <li class="text-center leading-7">Download <a class="underline" href="https://ntfy.sh">ntfy.sh</a> (web and mobile available)</li>
                    <li class="text-center leading-7">Subscribe to {"https://ntfy.tim.hackclub.app/username"}.</li>
                    <li class="text-center leading-7">{"Don't forget to save your setting"}</li>
                    <li class="text-center leading-7">If you have any problems ping or dm me on slack</li>
                    <li class="text-center leading-7">You can find me in the siege channel as Tim</li>
                </ul>
            </div>
        </div>
        <div class="col-start-3 row-start-1 justify-self-center pt-5 overflow-scroll w-full pr-12">
            <Suspense fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || {
                    project_loader
                        .get()
                        .map(|res| {
                            match res {
                                Ok(data) => {
                                    view! {
                                        <ul>
                                            {
                                                data.projects
                                                    .into_iter()
                                                    .map(|p| {
                                                        view! {
                                                            <li
                                                                class="bg-zinc-700 hover:bg-zinc-600 w-full hover:cursor-pointer flex justify-between items-center height-12 leading-12 rounded-[3rem] pl-12 pr-12 mb-3"
                                                                class:bg-yellow-600={
                                                                    let name = p.name.clone();
                                                                    move || primary.get() == name
                                                                }
                                                                class:bg-zinc-700={
                                                                    let name = p.name.clone();
                                                                    move || primary.get() != name
                                                                }
                                                                class:hover:bg-yellow-500={
                                                                    let name = p.name.clone();
                                                                    move || primary.get() == name
                                                                }
                                                                class:hover:bg-zinc-600={
                                                                    let name = p.name.clone();
                                                                    move || primary.get() != name
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
