use seed::prelude::Orders;

use crate::messages::{cards::CardsMsg, Msg};
use crate::state::entities::Card;

pub struct EditCardLinkScreenModel {
    pub loading: bool,
    pub selected_card: Option<Card>,
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
    action: &Msg,
    model: &mut EditCardLinkScreenModel,
    orders: &mut impl Orders<Msg>,
) {
    match action {
        Msg::Cards(CardsMsg::EditCardLink(_)) => {
            model.loading = true;
        }

        Msg::Cards(CardsMsg::EditCardLinkSuccess(_))
        | Msg::Cards(CardsMsg::EditCardLinkFailed(_)) => {
            model.loading = false;
        }

        Msg::Cards(CardsMsg::ViewEditCardLink(payload)) => {
            model.selected_card = Some(payload.card.clone());
        }

        _ => {}
    }
}
