use std::collections::HashMap;

use chrono::{Datelike, NaiveDate, Weekday};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let timeslots = use_state(|| vec![String::from("M1"), String::from("M2"), String::from("S1")]);
    let selected_timeslot = use_state(|| Option::<String>::None);
    let calendar = use_state(|| HashMap::<NaiveDate, String>::new());
    let year = use_state(|| 2024);
    let month = use_state(|| 1);

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
        Callback::from(move |date: NaiveDate| {
            let mut calendar_tmp = (*calendar).clone();
            match selected_timeslot.as_deref() {
                Some(timeslot) => calendar_tmp.insert(date, timeslot.to_string()),
                None => calendar_tmp.remove(&date),
            };
            calendar.set(calendar_tmp);
        })
    };

    html! {
        <>
            <div class={classes!("flex", "justify-between")}>
                <span><button onclick={previous_month} class={classes!("px-4", "py-2", "bg-blue-500", "text-white", "rounded-full")}>{"<"}</button></span>
                <span>{ format!("{:02}/{:04}", *month, *year) }</span>
                <span><button onclick={next_month} class={classes!("px-4", "py-2", "bg-blue-500", "text-white", "rounded-full")}>{">"}</button></span>
            </div>
            <Month year={*year} month={*month} calendar={(*calendar).clone()} {day_onclick}></Month>
            <div class={classes!("flex", "gap-2")}>
            { for timeslots.iter().cloned().map(|timeslot| {
                let timeslot_onclick = timeslot_onclick.clone();
                let timeslot_clone = timeslot.clone();
                let is_selected = selected_timeslot.as_ref().is_some_and(|selected| *selected == timeslot);
                html! {
                    <button class={classes!("px-4", "py-2", "bg-red-500", "rounded-full", is_selected.then_some("ring"))}
                        onclick={move |_| timeslot_onclick.emit(timeslot_clone.clone())}>{timeslot}</button>
            }}) }
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct MonthProps {
    year: i32,
    month: u32,
    calendar: HashMap<NaiveDate, String>,
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
            { for first_day.iter_days().take(42).map(|date| html! { <Day {date} timeslot={props.calendar.get(&date).cloned()} onclick={props.day_onclick.clone()}></Day> }) }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct DayProps {
    date: NaiveDate,
    timeslot: Option<String>,
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
            <p>{format!("{}", props.timeslot.clone().unwrap_or(".".to_string()))}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
