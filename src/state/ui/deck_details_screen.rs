use seed::prelude::Orders;

use crate::actions::{decks::DecksAction, GlobalAction};
use crate::state::entities::Deck;

pub struct DeckDetailsScreenModel {
    loading: bool,
    selected_deck: Option<Deck>,
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
    action: &GlobalAction,
    model: &mut DeckDetailsScreenModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Decks(DecksAction::DeleteDeck(_)) => {
            model.loading = true;
        }

        GlobalAction::Decks(DecksAction::DeleteDeckSuccess(_))
        | GlobalAction::Decks(DecksAction::DeleteDeckFailed(_)) => {
            model.loading = false;
        }

        GlobalAction::Decks(DecksAction::ViewDeckDetails(payload)) => {
            model.selected_deck = Some(payload.deck.clone());
        }

        GlobalAction::Decks(DecksAction::ViewDeckItems(payload)) => {
            model.selected_deck = Some(payload.deck.clone());
        }

        _ => {}
    }
}
