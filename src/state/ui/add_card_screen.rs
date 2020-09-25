use seed::prelude::Orders;

use crate::actions::{cards::CardsAction, GlobalAction};

pub struct AddCardScreenModel {
    loading: bool,
}

impl AddCardScreenModel {
    pub fn new() -> Self {
        Self { loading: false }
    }
}

pub fn update(
    action: &GlobalAction,
    model: &mut AddCardScreenModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Cards(CardsAction::AddCard(_)) => {
            model.loading = true;
        }

        GlobalAction::Cards(CardsAction::AddCardSuccess(_))
        | GlobalAction::Cards(CardsAction::AddCardFailed(_)) => {
            model.loading = false;
        }

        _ => {}
    }
}
