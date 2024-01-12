use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Month></Month>
        </>
    }
}

#[function_component]
fn Month() -> Html {
    html! {
        <div class={classes!("grid", "grid-cols-7", "justify-items-center", "text-center", "max-w-sm", "border-2", "rounded")}>
            { for (1..=42).map(|number| html! { <Day {number}></Day> }) }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct DayProps {
    number: u8,
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
