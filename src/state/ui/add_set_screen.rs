use seed::prelude::Orders;

use crate::actions::{sets::SetsAction, GlobalAction};
use crate::state::entities::Card;

pub struct AddSetScreenModel {
    cards: Vec<Card>,
    loading: bool,
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
    action: &GlobalAction,
    model: &mut AddSetScreenModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Sets(SetsAction::AddSet(_)) => {
            model.loading = true;
        }

        GlobalAction::Sets(SetsAction::AddSetSuccess(_))
        | GlobalAction::Sets(SetsAction::AddSetFailed(_)) => {
            model.loading = false;
        }

        GlobalAction::Sets(SetsAction::GotoAddSet(payload)) => {
            model.cards = payload.cards.clone();
        }

        _ => {}
    }
}
