use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::{
        sets::{AddSetPayload, SetsMsg},
        Msg,
    },
    state::Model,
};

#[topo::nested]
pub fn view(model: &Model, username: &str, deck_id: usize) -> Node<Msg> {
    if model.ui.add_card_screen.loading {
        return p!["loading..."];
    }

    if model.authentication.username.is_none()
        || model
            .authentication
            .username
            .as_ref()
            .map(|un| un.as_str())
            .unwrap()
            != username
    {
        return p![format!(
            "You must be logged in as {} to add sets for {}!",
            username, username
        )];
    }

    let name = use_state(String::new);
    let card_ids = (&model)
        .ui
        .add_set_screen
        .card_ids
        .iter()
        .cloned()
        .collect();

    div![
        h3!["Add set:"],
        "Name:",
        input![
            attrs! { At::Value => name },
            input_ev(Ev::Input, move |value| name.set(value))
        ],
        br![],
        br![],
        button![
            "Add",
            ev(Ev::Click, move |_| Msg::Sets(SetsMsg::AddSet(
                AddSetPayload {
                    deck_id,
                    name: name.get(),
                    card_ids,
                }
            )))
        ]
    ]
}
