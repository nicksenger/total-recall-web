use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::{
        decks::{AddDeckPayload, DecksMsg},
        Msg,
    },
    state::Model,
};

#[topo::nested]
pub fn view(model: &Model, username: &str) -> Node<Msg> {
    if model.ui.add_deck_screen.loading {
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
            "You must be logged in as {} to add decks for {}!",
            username, username
        )];
    }

    let deck_name = use_state(String::new);
    let language = use_state(|| {
        model
            .entities
            .languages
            .values()
            .next()
            .map(|l| l.id)
            .unwrap_or(0)
    });
    let username = (&model)
        .authentication
        .username
        .as_ref()
        .map(|un| un.as_str())
        .unwrap_or("")
        .to_owned();

    div![
        h3!["Add deck:"],
        "Name:",
        input![
            attrs! { At::Value => deck_name },
            input_ev(Ev::Input, move |value| deck_name.set(value))
        ],
        br![],
        br![],
        "Language:",
        select![
            attrs! { At::Value => language },
            input_ev(Ev::Change, move |value| language
                .set(value.parse::<usize>().unwrap_or(0))),
            model
                .entities
                .languages
                .values()
                .map(|l| option![attrs! { At::Value => l.id }, l.name.as_str()])
        ],
        br![],
        br![],
        button![
            "Add",
            ev(Ev::Click, move |_| Msg::Decks(DecksMsg::AddDeck(
                AddDeckPayload {
                    language: language.get(),
                    name: deck_name.get(),
                    username
                }
            )))
        ]
    ]
}
