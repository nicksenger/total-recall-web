use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::{
        decks::{DecksMsg, DeleteDeckPayload},
        Msg,
    },
    state::{entities::Deck, Model},
};

#[topo::nested]
pub fn view(model: &Model, deck_id: usize) -> Node<Msg> {
    if model.ui.deck_details_screen.loading {
        return p!["loading..."];
    }

    model
        .entities
        .decks
        .get(&deck_id)
        .map(deck_view)
        .unwrap_or(p!["Deck not loaded."])
}

fn deck_view(deck: &Deck) -> Node<Msg> {
    let deck_id = deck.id;
    div![
        label!["Name: ", strong![deck.name.to_owned()]],
        br![],
        label!["Language: ", strong![deck.language.to_owned()]],
        br![],
        br![],
        button![
            "Delete",
            ev(Ev::Click, move |_| Msg::Decks(DecksMsg::DeleteDeck(
                DeleteDeckPayload { deck_id }
            )))
        ]
    ]
}
