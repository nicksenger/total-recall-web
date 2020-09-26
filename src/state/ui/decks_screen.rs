use crate::messages::{decks::DecksMsg, Msg};

pub struct DecksScreenModel {
    pub decks: Vec<usize>,
    pub loading: bool,
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
    action: &Msg,
    model: &mut DecksScreenModel,
) {
    match action {
        Msg::Decks(DecksMsg::GetDecks(_)) => {
            model.loading = true;
        }

        Msg::Decks(DecksMsg::GetDecksSuccess(payload)) => {
            model.loading = false;
            model.decks = payload.decks.iter().map(|d| d.id).collect();
        }

        Msg::Decks(DecksMsg::GetDecksFailed(_)) => {
            model.loading = false;
        }

        Msg::Decks(DecksMsg::DeleteDeckSuccess(payload)) => {
            model.decks.retain(|&id| id != payload.deck_id);
        }

        _ => {}
    }
}
