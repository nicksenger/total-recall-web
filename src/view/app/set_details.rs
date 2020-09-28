use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::Msg,
    state::{entities::Set, routing::Route, Model},
};

#[topo::nested]
pub fn view(model: &Model, set_id: usize) -> Node<Msg> {
    if model.ui.set_details_screen.loading {
        return p!["loading..."];
    }

    model
        .entities
        .sets
        .get(&set_id)
        .map(set_view)
        .unwrap_or(p!["Set not loaded."])
}

fn set_view(set: &Set) -> Node<Msg> {
    div![label!["Name: ", strong![set.name.to_owned()]]]
}
