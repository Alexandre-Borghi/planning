use chrono::{Datelike, NaiveDate};
use gloo::net::http::Request;
use yew::{prelude::*, suspense::use_future};
use yew_hooks::prelude::*;

use crate::services::calendar;
use crate::types::{Days, Timeslots};
use month::Month;

mod day;
mod month;

#[function_component]
pub fn Calendar() -> HtmlResult {
    let timeslots = use_map(Timeslots::new());
    let calendar = use_map(Days::new());
    use_future(|| {
        let timeslots = timeslots.clone();
        let calendar = calendar.clone();
        async move {
            let path = "/api/timeslots";
            timeslots.set(Request::get(path).send().await.unwrap().json().await?);
            let path = "/api/calendar";
            calendar.set(Request::get(path).send().await.unwrap().json().await?);
            gloo::console::debug!(format!("calendar: {:?}", calendar.current()));
            Result::<(), gloo::net::Error>::Ok(())
        }
    })?;

    let selected_timeslot = use_state(|| Option::<String>::None);
    let is_editing = use_bool_toggle(false);
    let now = chrono::Local::now();
    let year = use_state(|| now.year());
    let month = use_state(|| now.month());

    let month_update = use_callback(
        (month.clone(), year.clone()),
        move |delta: i32, (month, year)| {
            let month_after_delta = **month as i32 + delta;
            let new_month = (month_after_delta - 1).rem_euclid(12) as u32 + 1;
            month.set(new_month);
            let year_delta = (month_after_delta - new_month as i32) / 12;
            year.set(**year + year_delta);
        },
    );

    let select_timeslot = use_callback(
        selected_timeslot.clone(),
        move |id: String, selected_timeslot| match **selected_timeslot {
            Some(ref selected) if *selected == id => selected_timeslot.set(None),
            _ => selected_timeslot.set(Some(id)),
        },
    );

    let day_onclick = use_callback(
        (
            calendar.clone(),
            selected_timeslot.clone(),
            is_editing.clone(),
        ),
        move |date: NaiveDate, (calendar, selected_timeslot, is_editing)| {
            if !**is_editing {
                return;
            }

            let calendar = calendar.clone();
            let selected_timeslot = selected_timeslot.clone();
            wasm_bindgen_futures::spawn_local(async move {
                calendar::update_day(date, selected_timeslot.as_deref()).await;
                match *selected_timeslot {
                    Some(ref timeslot) => calendar.insert(date, timeslot.clone()),
                    None => calendar.remove(&date),
                };
            })
        },
    );

    let toggle_edit_mode = use_callback(is_editing.clone(), move |_, is_editing| {
        is_editing.toggle();
    });

    let previous_month = month_update.reform(|_| -1);
    let next_month = month_update.reform(|_| 1);

    let button_cl = classes!("px-4", "py-2", "rounded-full");
    let button_primary_cl = classes!(button_cl.clone(), "bg-blue-500", "text-white");

    let make_timeslot_buttons = {
        let timeslots = timeslots.clone();
        let selected_timeslot = selected_timeslot.clone();
        let select_timeslot = select_timeslot.clone();
        move || {
            timeslots
                .current()
                .iter()
                .map(|(timeslot, color)| {
                    let is_selected = selected_timeslot
                        .as_ref()
                        .is_some_and(|selected| *selected == *timeslot);
                    let select_timeslot = {
                        let timeslot = timeslot.clone();
                        select_timeslot.reform(move |_| timeslot.clone())
                    };
                    timeslot_button(timeslot, color, is_selected, select_timeslot)
                })
                .collect::<Html>()
        }
    };

    Ok(html! {
        <>
            <div class={classes!("flex", "justify-between")}>
                <span>
                    <button onclick={previous_month}
                            class={button_primary_cl.clone()}>
                        {"<"}
                    </button>
                </span>
                <span>{ format!("{:02}/{:04}", *month, *year) }</span>
                <span>
                    <button onclick={next_month} class={button_primary_cl}>
                        {">"}
                    </button>
                </span>
            </div>

            <Month year={*year} month={*month}
                   calendar={calendar.current().clone()}
                   timeslots={timeslots.current().clone()}
                   {day_onclick}>
            </Month>

            <div class={classes!("flex", "gap-2")}>
                <button onclick={toggle_edit_mode}
                        class={classes!(button_cl.clone(), "border")}>
                    {"Edit"}
                </button>
                {is_editing.then(make_timeslot_buttons)}
            </div>
        </>
    })
}

fn timeslot_button(
    timeslot: &str,
    color: &str,
    is_selected: bool,
    onclick: Callback<MouseEvent>,
) -> Html {
    html! {
        <button {onclick}
                class={classes!("px-4", "py-2", "rounded-full", is_selected.then_some("ring"))}
                style={format!("background-color: {}", color)}>
            {timeslot}
        </button>
    }
}
