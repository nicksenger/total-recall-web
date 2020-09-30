use seed::prelude::*;

use crate::{messages::Msg, state::Model};

pub mod authentication;
pub mod cache;
pub mod cards;
pub mod decks;
pub mod routing;
pub mod session;
pub mod sets;

pub fn operate(msg: &Msg, model: &Model, orders: &mut impl Orders<Msg>) {
    authentication::operate(msg, model, orders);
    cache::operate(msg, model, orders);
    cards::operate(msg, model, orders);
    decks::operate(msg, model, orders);
    routing::operate(msg, model, orders);
    session::operate(msg, model, orders);
    sets::operate(msg, model, orders);
}
