use std::collections::HashMap;

use yew::{
    prelude::*,
    suspense::{use_future, UseFutureHandle},
};
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Config() -> HtmlResult {
    let timeslots = use_map(HashMap::new());
    let _: UseFutureHandle<Result<_, gloo::net::Error>> = use_future(|| {
        let timeslots = timeslots.clone();
        async move {
            let timeslots_json: HashMap<String, String> =
                gloo::net::http::Request::get("/api/timeslots")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await?;
            timeslots.set(timeslots_json);
            Ok(())
        }
    })?;

    let timeslots_sorted = timeslots.current().clone();
    let mut timeslots_sorted = timeslots_sorted.iter().collect::<Vec<_>>();
    timeslots_sorted.sort_unstable();

    Ok(html! {
        <>
        <h1 class={classes!("mb-4", "text-4xl", "font-extrabold")}>{"Configuration"}</h1>
        <ul class={classes!("list-disc", "list-inside")}>
        { for timeslots_sorted.iter().map(|(timeslot, color)| {
        html! {
            <li>
                <Link<Route> to={Route::EditTimeslot { timeslot_id: AttrValue::from(timeslot.to_string()) }}>
                    {timeslot}{" : "}{color}
                </Link<Route>>
            </li>
        }}) }
        </ul>
        </>
    })
}
