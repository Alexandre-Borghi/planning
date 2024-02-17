use yew::{
    prelude::*,
    suspense::{use_future, UseFutureHandle},
};
use yew_router::prelude::*;

use crate::{types::Timeslots, Route};

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

    Ok(html! {
        <>
        <h1 class={classes!("mb-4", "text-4xl", "font-extrabold")}>{"Modifier "}{props.timeslot_id.clone()}</h1>
        <form>
            <div>
                <label>
                    {"Nom : "}
                    <input type={"text"} value={(*timeslot_id).clone()}
                        class={classes!("border", "border-black")} />
                </label>
            </div>
            <div>
                <label>
                    {"Couleur : "}
                    <input type={"color"} value={(*timeslot_color).clone()}/>
                </label>
            </div>
            <div>
                <Link<Route> to={Route::Config}>{"Annuler"}</Link<Route>>
                <button>{"Confirmer"}</button>
            </div>
        </form>
        </>
    })
}
