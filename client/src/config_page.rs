use std::collections::HashMap;

use yew::{
    prelude::*,
    suspense::{use_future, UseFutureHandle},
};
use yew_hooks::prelude::*;

#[function_component]
pub fn ConfigPage() -> HtmlResult {
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

    Ok(html! {
        <>
        <h1 class={classes!("mb-4", "text-4xl", "font-extrabold")}>{"Configuration"}</h1>
        <ul class={classes!("list-disc", "list-inside")}>
        { for timeslots.current().iter().map(|(timeslot, color)| {
        html! {
            <li>{timeslot}{" : "}{color}</li>
        }}) }
        </ul>
        </>
    })
}
