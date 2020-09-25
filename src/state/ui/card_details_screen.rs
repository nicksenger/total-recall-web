use seed::prelude::Orders;

use crate::actions::{cards::CardsAction, GlobalAction};
use crate::state::entities::Card;

pub struct CardDetailsScreenModel {
    loading: bool,
    selected_card: Option<Card>,
}

impl CardDetailsScreenModel {
    pub fn new() -> Self {
        Self {
            loading: false,
            selected_card: None,
        }
    }
}

pub fn update(
    action: &GlobalAction,
    model: &mut CardDetailsScreenModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Cards(CardsAction::DeleteCard(_)) => {
            model.loading = true;
        }

        GlobalAction::Cards(CardsAction::DeleteCardSuccess(_))
        | GlobalAction::Cards(CardsAction::DeleteCardFailed(_)) => {
            model.loading = false;
        }

        GlobalAction::Cards(CardsAction::EditCardLinkSuccess(payload)) => {
            model
                .selected_card
                .as_mut()
                .map(|c| c.link = Some(payload.link.clone()));
        }

        GlobalAction::Cards(CardsAction::ViewCardDetails(payload)) => {
            model.selected_card = Some(payload.card.clone());
        }

        _ => {}
    }
}
