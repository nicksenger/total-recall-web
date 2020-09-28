use seed::prelude::*;
use seed_hooks::*;

use crate::{
    messages::Msg,
    state::{routing::Route, Model},
};

mod app;
mod auth;
mod home;
mod not_found;

#[topo::nested]
pub fn root(model: &Model) -> Node<Msg> {
    match model.routing.route {
        Route::Home => home::view(model),
        Route::Login | Route::Register => auth::view(model),
        Route::NotFound => not_found::view(model),
        _ => app::view(model),
    }
}
