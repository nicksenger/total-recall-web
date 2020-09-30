use crate::messages::{sets::SetsMsg, Msg};

pub struct AddSetScreenModel {
    pub card_ids: Vec<usize>,
    pub loading: bool,
}

impl AddSetScreenModel {
    pub fn new() -> Self {
        Self {
            card_ids: vec![],
            loading: false,
        }
    }
}

pub fn update(action: &Msg, model: &mut AddSetScreenModel) {
    match action {
        Msg::Sets(SetsMsg::AddSet(_)) => {
            model.loading = true;
        }

        Msg::Sets(SetsMsg::AddSetSuccess(_)) | Msg::Sets(SetsMsg::AddSetFailed(_)) => {
            model.loading = false;
        }

        Msg::Sets(SetsMsg::GotoAddSet(payload)) => {
            model.card_ids = payload.cards.clone();
        }

        _ => {}
    }
}
