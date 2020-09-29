use seed::{prelude::*, *};

use crate::{
    messages::Msg,
    state::{routing::Route, Model},
};

mod add_card;
mod add_deck;
mod add_set;
mod card_details;
mod deck_cards;
mod deck_details;
mod deck_sets;
mod decks;
mod set_details;

pub fn view(model: &Model) -> Node<Msg> {
    match &model.routing.route {
        Route::Decks(username) => decks::view(&model, username.as_str()),
        Route::DeckCards(username, deck_id) => deck_cards::view(&model, username, *deck_id),
        Route::DeckSets(username, deck_id) => deck_sets::view(&model, username, *deck_id),
        Route::CardDetails(card_id) => card_details::view(&model, *card_id),
        Route::SetDetails(set_id) => set_details::view(&model, *set_id),
        Route::AddDeck(username) => add_deck::view(&model, username.as_str()),
        Route::DeckDetails(_username, deck_id) => deck_details::view(&model, *deck_id),
        Route::AddCard(username, deck_id) => add_card::view(&model, username.as_str(), *deck_id),
        Route::AddSet(username, deck_id) => add_set::view(&model, username.as_str(), *deck_id),
        _ => h3!["Invalid route."],
    }
}
