use yew::{
    prelude::*,
    suspense::{use_future, UseFutureHandle},
};
use yew_router::prelude::*;

use crate::{services::timeslots, types::Timeslots, Route};

#[derive(Debug, Default, Properties, PartialEq)]
pub struct Props {
    pub timeslot_id: AttrValue,
}

#[function_component]
pub fn EditTimeslot(props: &Props) -> HtmlResult {
    let timeslot_id = use_state(|| props.timeslot_id.clone());
    let timeslot_color = use_state(String::new);
    let _: UseFutureHandle<Result<_, gloo::net::Error>> = use_future(|| {
        let timeslot_id = timeslot_id.clone();
        let timeslot_color = timeslot_color.clone();
        async move {
            let timeslots_json: Timeslots = gloo::net::http::Request::get("/api/timeslots")
                .send()
                .await
                .unwrap()
                .json()
                .await?;
            timeslot_color.set(
                timeslots_json
                    .get(&timeslot_id.to_string())
                    .unwrap_or_else(|| panic!("unknown timeslot {timeslot_id:?}"))
                    .to_string(),
            );
            Ok(())
        }
    })?;

    let onconfirm = use_callback(
        (timeslot_id.clone(), timeslot_color.clone()),
        move |e: MouseEvent, (timeslot_id, timeslot_color)| {
            e.prevent_default();
            let timeslot_id = timeslot_id.clone();
            let timeslot_color = timeslot_color.clone();
            wasm_bindgen_futures::spawn_local(async move {
                timeslots::update(&timeslot_id, &timeslot_color).await;
            })
        },
    );

    Ok(html! {
        <>
        <h1 class={classes!("mb-4", "text-4xl", "font-extrabold")}>{"Modifier "}{props.timeslot_id.clone()}</h1>
        <form>
            <div>
                <label>
                    {"Couleur : "}
                    <input type={"color"} value={(*timeslot_color).clone()} />
                </label>
            </div>
            <div>
                <Link<Route> to={Route::Config}>{"Annuler"}</Link<Route>>
                <button onclick={onconfirm}>{"Confirmer"}</button>
            </div>
        </form>
        </>
    })
}
