use seed::prelude::Orders;

use crate::actions::{cards::CardsAction, GlobalAction};
use crate::state::entities::Card;

pub struct EditCardLinkScreenModel {
    loading: bool,
    selected_card: Option<Card>,
}

impl EditCardLinkScreenModel {
    pub fn new() -> Self {
        Self {
            loading: false,
            selected_card: None,
        }
    }
}

pub fn update(
    action: &GlobalAction,
    model: &mut EditCardLinkScreenModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Cards(CardsAction::EditCardLink(_)) => {
            model.loading = true;
        }

        GlobalAction::Cards(CardsAction::EditCardLinkSuccess(_))
        | GlobalAction::Cards(CardsAction::EditCardLinkFailed(_)) => {
            model.loading = false;
        }

        GlobalAction::Cards(CardsAction::ViewEditCardLink(payload)) => {
            model.selected_card = Some(payload.card.clone());
        }

        _ => {}
    }
}
