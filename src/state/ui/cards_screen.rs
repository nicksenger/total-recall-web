use crate::messages::{cards::CardsMsg, Msg};

pub struct CardsScreenModel {
    pub cards: Vec<usize>,
    pub loading: bool,
}

impl CardsScreenModel {
    pub fn new() -> Self {
        Self {
            cards: vec![],
            loading: false,
        }
    }
}

pub fn update(action: &Msg, model: &mut CardsScreenModel) {
    match action {
        Msg::Cards(CardsMsg::GetCards(_)) => {
            model.loading = true;
        }

        Msg::Cards(CardsMsg::GetCardsSuccess(_)) | Msg::Cards(CardsMsg::GetCardsFailed(_)) => {
            model.loading = false;
        }

        Msg::Cards(CardsMsg::DeleteCardSuccess(payload)) => {
            model.cards.retain(|&id| id != payload.card_id)
        }

        _ => {}
    }
}
