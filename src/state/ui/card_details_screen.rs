use crate::messages::{cards::CardsMsg, Msg};

pub struct CardDetailsScreenModel {
    pub loading: bool,
}

impl CardDetailsScreenModel {
    pub fn new() -> Self {
        Self {
            loading: false,
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

        _ => {}
    }
}
