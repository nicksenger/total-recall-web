use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::{
        sets::{DeleteSetPayload, SetsMsg},
        Msg,
    },
    state::{entities::Set, Model},
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
    let set_id = set.id;

    div![
        label!["Name: ", strong![set.name.to_owned()]],
        br![],
        br![],
        button![
            "Delete",
            ev(Ev::Click, move |_| Msg::Sets(SetsMsg::DeleteSet(
                DeleteSetPayload { set_id }
            )))
        ]
    ]
}
