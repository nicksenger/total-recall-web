use seed::prelude::Orders;

use crate::messages::{decks::DecksMsg, Msg};
use crate::state::entities::Deck;

pub struct DeckDetailsScreenModel {
    pub loading: bool,
    pub selected_deck: Option<Deck>,
}

impl DeckDetailsScreenModel {
    pub fn new() -> Self {
        Self {
            loading: false,
            selected_deck: None,
        }
    }
}

pub fn update(
    action: &Msg,
    model: &mut DeckDetailsScreenModel,
    orders: &mut impl Orders<Msg>,
) {
    match action {
        Msg::Decks(DecksMsg::DeleteDeck(_)) => {
            model.loading = true;
        }

        Msg::Decks(DecksMsg::DeleteDeckSuccess(_))
        | Msg::Decks(DecksMsg::DeleteDeckFailed(_)) => {
            model.loading = false;
        }

        Msg::Decks(DecksMsg::ViewDeckDetails(payload)) => {
            model.selected_deck = Some(payload.deck.clone());
        }

        Msg::Decks(DecksMsg::ViewDeckItems(payload)) => {
            model.selected_deck = Some(payload.deck.clone());
        }

        _ => {}
    }
}
