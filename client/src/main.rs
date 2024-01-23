use std::collections::HashMap;

use chrono::{Datelike, NaiveDate, Weekday};
use yew::{
    prelude::*,
    suspense::{use_future, UseFutureHandle},
};
use yew_hooks::prelude::*;
use yew_router::prelude::*;

mod config_page;

use config_page::ConfigPage;

#[function_component]
fn App() -> HtmlResult {
    let timeslots = use_map(HashMap::new());
    let calendar = use_map(HashMap::<NaiveDate, String>::new());
    let _: UseFutureHandle<Result<_, gloo::net::Error>> = use_future(|| {
        let timeslots = timeslots.clone();
        let calendar = calendar.clone();
        async move {
            let timeslots_json: HashMap<String, String> =
                gloo::net::http::Request::get("/api/timeslots")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await?;
            timeslots.set(timeslots_json);
            let calendar_json: HashMap<NaiveDate, String> =
                gloo::net::http::Request::get("/api/calendar")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await?;
            gloo::console::debug!(format!("calendar: {calendar_json:?}"));
            calendar.set(calendar_json);
            Ok(())
        }
    })?;
    let selected_timeslot = use_state(|| Option::<String>::None);
    let is_editing = use_state(|| false);
    let now = chrono::Local::now();
    let year = use_state(|| now.year());
    let month = use_state(|| now.month());

    let previous_month = {
        let year = year.clone();
        let month = month.clone();
        move |_| {
            let new_year = if *month == 1 { *year - 1 } else { *year };
            let new_month = if *month == 1 { 12 } else { *month - 1 };
            year.set(new_year);
            month.set(new_month);
        }
    };

    let next_month = {
        let year = year.clone();
        let month = month.clone();
        move |_| {
            let new_year = if *month == 12 { *year + 1 } else { *year };
            let new_month = if *month == 12 { 1 } else { *month + 1 };
            year.set(new_year);
            month.set(new_month);
        }
    };

    let timeslot_onclick = {
        let selected_timeslot = selected_timeslot.clone();
        Callback::from(move |id: String| {
            if selected_timeslot
                .as_ref()
                .is_some_and(|selected| *selected == id)
            {
                selected_timeslot.set(None);
            } else {
                selected_timeslot.set(Some(id));
            }
        })
    };

    let day_onclick = {
        let calendar = calendar.clone();
        let selected_timeslot = selected_timeslot.clone();
        let is_editing = is_editing.clone();
        Callback::from(move |date: NaiveDate| {
            if !*is_editing {
                return;
            }

            let calendar = calendar.clone();
            let selected_timeslot = selected_timeslot.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let body = format!(
                    "{{\"date\": \"{date}\", \"timeslot\": {}}}",
                    selected_timeslot
                        .as_deref()
                        .map(|s| format!("\"{s}\""))
                        .unwrap_or("null".to_string())
                );

                gloo::net::http::Request::post("/api/calendar")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .unwrap()
                    .send()
                    .await
                    .expect("post request to calendar returned an error");

                match selected_timeslot.as_deref() {
                    Some(timeslot) => calendar.insert(date, timeslot.to_string()),
                    None => calendar.remove(&date),
                };
            })
        })
    };

    let toggle_edit_mode = {
        let is_editing = is_editing.clone();
        move |_| {
            is_editing.set(!*is_editing);
        }
    };

    Ok(html! {
        <>
            <div class={classes!("flex", "justify-between")}>
                <span><button onclick={previous_month} class={classes!("px-4", "py-2", "bg-blue-500", "text-white", "rounded-full")}>{"<"}</button></span>
                <span>{ format!("{:02}/{:04}", *month, *year) }</span>
                <span><button onclick={next_month} class={classes!("px-4", "py-2", "bg-blue-500", "text-white", "rounded-full")}>{">"}</button></span>
            </div>
            <Month year={*year} month={*month} calendar={calendar.current().clone()} timeslots={timeslots.current().clone()} {day_onclick}></Month>
            <div class={classes!("flex", "gap-2")}>
                <button onclick={toggle_edit_mode} class={classes!("px-4", "py-2", "border", "rounded-full")}>{"Edit"}</button>
            if *is_editing {
            { for timeslots.current().iter().map(|(timeslot, color)| {
                let timeslot_onclick = timeslot_onclick.clone();
                let timeslot_clone = timeslot.clone();
                let is_selected = selected_timeslot.as_ref().is_some_and(|selected| *selected == *timeslot);
                html! {
                    <button class={classes!("px-4", "py-2", "rounded-full", is_selected.then_some("ring"))}
                        style={format!("background-color: {}", *color)}
                        onclick={move |_| timeslot_onclick.emit(timeslot_clone.clone())}>{timeslot}</button>
            }}) }
            }
            </div>
        </>
    })
}

#[derive(Properties, PartialEq)]
struct MonthProps {
    year: i32,
    month: u32,
    calendar: HashMap<NaiveDate, String>,
    timeslots: HashMap<String, String>,
    day_onclick: Callback<NaiveDate>,
}

#[function_component]
fn Month(props: &MonthProps) -> Html {
    let first_day = NaiveDate::from_ymd_opt(props.year, props.month, 1)
        .unwrap()
        .week(Weekday::Mon)
        .first_day();

    html! {
        <div class={classes!("grid", "grid-cols-7", "justify-items-center", "text-center", "max-w-sm", "border-2", "rounded")}>
            { for ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"].map(|name| html! {
                <div class={classes!("w-full", "border", "font-bold")}>
                    {name}
                </div>
            })}
            { for first_day.iter_days().take(42).map(|date| {
                let timeslot = props.calendar.get(&date).cloned();
                let color = timeslot.as_ref().map(|timeslot| props.timeslots.get(timeslot).cloned()).unwrap_or(Some("#ffffff00".to_string())).unwrap_or("#ff0000".to_string());
                html! { <Day {date} {timeslot} {color} onclick={props.day_onclick.clone()}></Day> }
            }) }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct DayProps {
    date: NaiveDate,
    timeslot: Option<String>,
    color: String,
    onclick: Callback<NaiveDate>,
}

#[function_component]
fn Day(props: &DayProps) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        let date = props.date;
        move |_| {
            onclick.emit(date);
        }
    };

    html! {
        <div class={classes!("w-full", "border", "select-none")} {onclick}>
            <p>{props.date.day()}</p>
            <p style={format!("background-color: {}", props.color)}>{format!("{}", props.timeslot.clone().unwrap_or(".".to_string()))}</p>
        </div>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/config")]
    Config,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <App></App> },
        Route::Config => html! { <ConfigPage></ConfigPage> },
    }
}

#[function_component]
fn SuspenseApp() -> Html {
    let fallback = html! { "Loading..." };

    html! {
        <Suspense {fallback}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
                <nav>
                    <Link<Route> to={Route::Home}>{"Accueil"}</Link<Route>>
                    <Link<Route> to={Route::Config}>{"Configuration"}</Link<Route>>
                </nav>
            </BrowserRouter>
        </Suspense>
    }
}

fn main() {
    yew::Renderer::<SuspenseApp>::new().render();
}
