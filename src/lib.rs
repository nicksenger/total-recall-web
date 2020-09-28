use std::convert::TryFrom;

use seed::{prelude::*, *};

use crate::{
    messages::{routing::RoutingMessage, Msg},
    state::routing::Route,
};

mod messages;
mod operations;
mod state;
mod utilities;
mod view;

pub const BASE_URI: &str = "https://trc.nsenger.com";

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, state::update, view::root);
}

fn init(url: Url, orders: &mut impl Orders<messages::Msg>) -> state::Model {
    orders
        .subscribe(|subs::UrlChanged(url)| {
            Msg::Routing(RoutingMessage::Navigate(
                Route::try_from(url).unwrap_or(Route::NotFound),
            ))
        })
        .notify(subs::UrlChanged(url.clone()));

    state::Model::new(url)
}
