use yew::prelude::*;
use yew_router::prelude::*;

use pages::{calendar::Calendar, config::Config};

mod pages;
mod services;
mod types;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Calendar,
    #[at("/config")]
    Config,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Calendar => html! { <Calendar></Calendar> },
        Route::Config => html! { <Config></Config> },
    }
}

#[function_component]
fn App() -> Html {
    let fallback = html! { "Loading..." };

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
