use seed::prelude::*;
use seed_hooks::*;

use crate::{
    components::app_frame,
    messages::Msg,
    state::{routing::Route, Model},
};

mod app;
mod auth;
mod home;
mod not_found;
mod user_manual;

#[topo::nested]
pub fn root(model: &Model) -> Node<Msg> {
    app_frame(
        model,
        vec![match model.routing.route {
            Route::Home => home::view(model),
            Route::Login | Route::Register => auth::view(model),
            Route::Manual => user_manual::view(model),
            Route::NotFound => not_found::view(model),
            _ => app::view(model),
        }],
    )
}
