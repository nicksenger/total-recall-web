use crate::messages::{sets::SetsMsg, Msg};
use crate::state::entities::Card;

pub struct AddSetScreenModel {
    pub cards: Vec<Card>,
    pub loading: bool,
}

impl AddSetScreenModel {
    pub fn new() -> Self {
        Self {
            cards: vec![],
            loading: false,
        }
    }
}

pub fn update(
    action: &Msg,
    model: &mut AddSetScreenModel,
) {
    match action {
        Msg::Sets(SetsMsg::AddSet(_)) => {
            model.loading = true;
        }

        Msg::Sets(SetsMsg::AddSetSuccess(_))
        | Msg::Sets(SetsMsg::AddSetFailed(_)) => {
            model.loading = false;
        }

        Msg::Sets(SetsMsg::GotoAddSet(payload)) => {
            model.cards = payload.cards.clone();
        }

        _ => {}
    }
}
