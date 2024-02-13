use std::collections::HashMap;

use chrono::{NaiveDate, Weekday};
use yew::prelude::*;

use crate::pages::calendar::day::Day;

#[derive(Properties, PartialEq)]
pub struct MonthProps {
    pub year: i32,
    pub month: u32,
    pub calendar: HashMap<NaiveDate, String>,
    pub timeslots: HashMap<String, String>,
    pub day_onclick: Callback<NaiveDate>,
}

#[function_component]
pub fn Month(props: &MonthProps) -> Html {
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
