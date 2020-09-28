use seed::{prelude::*, *};

use crate::{
    messages::Msg,
    state::{routing::Route, Model},
};

mod register;
mod sign_in;

pub fn view(model: &Model) -> Node<Msg> {
    match model.routing.route {
        Route::Register => register::view(model),
        Route::Login => sign_in::view(model),
        _ => h3!["Invalid route."],
    }
}
