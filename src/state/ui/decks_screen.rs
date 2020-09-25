use seed::prelude::Orders;

use crate::actions::{decks::DecksAction, GlobalAction};

pub struct DecksScreenModel {
    decks: Vec<usize>,
    loading: bool,
}

impl DecksScreenModel {
    pub fn new() -> Self {
        Self {
            loading: false,
            decks: vec![],
        }
    }
}

pub fn update(
    action: &GlobalAction,
    model: &mut DecksScreenModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Decks(DecksAction::GetDecks(_)) => {
            model.loading = true;
        }

        GlobalAction::Decks(DecksAction::GetDecksSuccess(payload)) => {
            model.loading = false;
            model.decks = payload.decks.iter().map(|d| d.id).collect();
        }

        GlobalAction::Decks(DecksAction::GetDecksFailed(_)) => {
            model.loading = false;
        }

        GlobalAction::Decks(DecksAction::DeleteDeckSuccess(payload)) => {
            model.decks.retain(|&id| id != payload.deck_id);
        }

        _ => {}
    }
}
