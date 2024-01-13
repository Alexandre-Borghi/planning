use yew::prelude::*;
use chrono::{NaiveDate, Weekday, Datelike};

#[function_component]
fn App() -> Html {
    let timeslots = use_state(|| vec![String::from("M1"), String::from("M2"), String::from("S1")]);
    let selected_timeslot = use_state(|| Option::<String>::None);
    let year = use_state(|| 2024);
    let month = use_state(|| 1);

    let onclick = {
        let selected_timeslot = selected_timeslot.clone();
        Callback::from(move |id: String| {
            if selected_timeslot.as_ref().is_some_and(|selected| *selected == id) {
                selected_timeslot.set(None);
            } else {
                selected_timeslot.set(Some(id));
            }
        })
    };

    html! {
        <>
            { format!("{:02}/{:04}", *month, *year) }
            <Month year={*year} month={*month}></Month>
            <div class={classes!("flex", "gap-2")}>
            { for timeslots.iter().cloned().map(|timeslot| {
                let onclick = onclick.clone();
                let timeslot_clone = timeslot.clone();
                let is_selected = selected_timeslot.as_ref().is_some_and(|selected| *selected == timeslot);
                html! {
                    <button class={classes!("px-4", "py-2", "bg-red-500", "rounded-full", is_selected.then_some("ring"))}
                        onclick={move |_| onclick.emit(timeslot_clone.clone())}>{timeslot}</button>
            }}) }
            </div>
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
