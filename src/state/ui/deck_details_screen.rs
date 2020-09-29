use crate::messages::{decks::DecksMsg, Msg};

pub struct DeckDetailsScreenModel {
    pub loading: bool,
}

impl DeckDetailsScreenModel {
    pub fn new() -> Self {
        Self {
            loading: false,
        }
    }
}

pub fn update(
    action: &Msg,
    model: &mut DeckDetailsScreenModel,
) {
    match action {
        Msg::Decks(DecksMsg::DeleteDeck(_)) => {
            model.loading = true;
        }

        Msg::Decks(DecksMsg::DeleteDeckSuccess(_))
        | Msg::Decks(DecksMsg::DeleteDeckFailed(_)) => {
            model.loading = false;
        }

        _ => {}
    }
}
