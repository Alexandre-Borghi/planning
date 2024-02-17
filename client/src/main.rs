use yew::prelude::*;
use yew_router::prelude::*;

use pages::{Calendar, Config, EditTimeslot};

mod pages;
mod services;
mod types;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Calendar,
    #[at("/config")]
    Config,
    #[at("/config/:timeslot_id")]
    EditTimeslot { timeslot_id: AttrValue },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Calendar => html! { <Calendar></Calendar> },
        Route::Config => html! { <Config></Config> },
        Route::EditTimeslot { timeslot_id } => {
            html! { <EditTimeslot {timeslot_id}></EditTimeslot> }
        }
    }
}

#[function_component]
fn App() -> Html {
    let fallback = html! { "Chargement..." };

    html! {
        <Suspense {fallback}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
                <nav>
                    <Link<Route> to={Route::Calendar}><img src="/img/icon-home.svg" /></Link<Route>>
                    <Link<Route> to={Route::Config}><img src="/img/icon-settings.svg" /></Link<Route>>
                </nav>
            </BrowserRouter>
        </Suspense>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
