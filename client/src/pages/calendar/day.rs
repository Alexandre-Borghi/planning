use chrono::{Datelike, NaiveDate};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DayProps {
    pub date: NaiveDate,
    pub timeslot: Option<String>,
    pub color: String,
    pub onclick: Callback<NaiveDate>,
}

#[function_component]
pub fn Day(props: &DayProps) -> Html {
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
