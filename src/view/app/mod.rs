use seed::{prelude::*, *};

use crate::{
    messages::Msg,
    state::{routing::Route, Model},
};

mod deck_cards;
mod deck_sets;
mod decks;

pub fn view(model: &Model) -> Node<Msg> {
    match &model.routing.route {
        Route::Decks(username) => decks::view(&model, username.as_str()),
        Route::DeckCards(username, deck_id) => deck_cards::view(&model, username, *deck_id),
        Route::DeckSets(username, deck_id) => deck_sets::view(&model, username, *deck_id),
        _ => h3!["Invalid route."],
    }
}
