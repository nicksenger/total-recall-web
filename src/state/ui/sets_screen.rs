use crate::messages::{sets::SetsMsg, Msg};

pub struct SetsScreenModel {
    pub loading: bool,
    pub sets: Vec<usize>,
}

impl SetsScreenModel {
    pub fn new() -> Self {
        Self {
            loading: false,
            sets: vec![],
        }
    }
}

pub fn update(action: &Msg, model: &mut SetsScreenModel) {
    match action {
        Msg::Sets(SetsMsg::GetSets(_)) => {
            model.loading = true;
        }

        Msg::Sets(SetsMsg::GetSetsFailed(_)) => {
            model.loading = false;
        }

        Msg::Sets(SetsMsg::GetSetsSuccess(payload)) => {
            model.loading = false;
            model.sets = payload.sets.iter().map(|s| s.id).collect();
        }

        Msg::Sets(SetsMsg::DeleteSetSuccess(payload)) => {
            model.sets.retain(|&id| id != payload.set_id);
        }

        _ => {}
    }
}
