use seed::prelude::Orders;

use crate::actions::{cards::CardsAction, GlobalAction};

pub struct CardsScreenModel {
    cards: Vec<usize>,
    loading: bool,
}

impl CardsScreenModel {
    pub fn new() -> Self {
        Self {
            cards: vec![],
            loading: false,
        }
    }
}

pub fn update(
    action: &GlobalAction,
    model: &mut CardsScreenModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Cards(CardsAction::GetCards(_)) => {
            model.loading = true;
        }

        GlobalAction::Cards(CardsAction::GetCardsSuccess(_))
        | GlobalAction::Cards(CardsAction::GetCardsFailed(_)) => {
            model.loading = false;
        }

        GlobalAction::Cards(CardsAction::DeleteCardSuccess(payload)) => {
            model.cards.retain(|&id| id != payload.card_id)
        }

        _ => {}
    }
}
