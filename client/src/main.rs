use yew::prelude::*;
use chrono::{NaiveDate, Weekday, Datelike};

#[function_component]
fn App() -> Html {
    let year = use_state(|| 2024);
    let month = use_state(|| 1);

    html! {
        <>
            { format!("{:02}/{:04}", *month, *year) }
            <Month year={*year} month={*month}></Month>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct MonthProps {
    year: i32,
    month: u32,
}

#[function_component]
fn Month(props: &MonthProps) -> Html {
    let first_day = NaiveDate::from_ymd_opt(props.year, props.month, 1).unwrap().week(Weekday::Mon).first_day();

    html! {
        <div class={classes!("grid", "grid-cols-7", "justify-items-center", "text-center", "max-w-sm", "border-2", "rounded")}>
            { for ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"].map(|name| html! {
                <div class={classes!("w-full", "border", "font-bold")}>
                    {name}
                </div>
            })}
            { for first_day.iter_days().take(42).map(|date| html! { <Day number={date.day()}></Day> }) }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct DayProps {
    number: u32,
}

#[function_component]
fn Day(props: &DayProps) -> Html {
    html! {
        <div class={classes!("w-full", "border")}>
            <p>{props.number}</p>
            <p>{"XX"}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
