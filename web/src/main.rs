use std::ops::{Deref, DerefMut};

use serde::Serialize;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::Routable;

use imprint::Imprint;

use crate::home::Home;
use crate::info::Info;

mod info;
mod home;
mod use_fetch;
mod form;
mod header;
mod permanent_redirect_checkbox;
mod imprint;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/info/:link")]
    Link { link: String },
    #[at("/imprint")]
    Imprint,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home/> },
        Route::Link { link } => html! {<Info link={link.clone()}/>},
        Route::Imprint {} => html! {<Imprint/>}
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
