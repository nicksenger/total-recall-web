use crate::messages::{cards::CardsMsg, Msg};
use crate::state::entities::Card;

pub struct CardDetailsScreenModel {
    pub loading: bool,
    pub selected_card: Option<Card>,
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
    action: &Msg,
    model: &mut CardDetailsScreenModel,
) {
    match action {
        Msg::Cards(CardsMsg::DeleteCard(_)) => {
            model.loading = true;
        }

        Msg::Cards(CardsMsg::DeleteCardSuccess(_))
        | Msg::Cards(CardsMsg::DeleteCardFailed(_)) => {
            model.loading = false;
        }

        Msg::Cards(CardsMsg::EditCardLinkSuccess(payload)) => {
            model
                .selected_card
                .as_mut()
                .map(|c| c.link = Some(payload.link.clone()));
        }

        Msg::Cards(CardsMsg::ViewCardDetails(payload)) => {
            model.selected_card = Some(payload.card.clone());
        }

        _ => {}
    }
}
