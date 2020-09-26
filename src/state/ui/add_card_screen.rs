use crate::messages::{cards::CardsMsg, Msg};

pub struct AddCardScreenModel {
    pub loading: bool,
}

impl AddCardScreenModel {
    pub fn new() -> Self {
        Self { loading: false }
    }
}

pub fn update(
    action: &Msg,
    model: &mut AddCardScreenModel,
) {
    match action {
        Msg::Cards(CardsMsg::AddCard(_)) => {
            model.loading = true;
        }

        Msg::Cards(CardsMsg::AddCardSuccess(_))
        | Msg::Cards(CardsMsg::AddCardFailed(_)) => {
            model.loading = false;
        }

        _ => {}
    }
}
