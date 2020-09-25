use seed::prelude::Orders;

use crate::actions::{sets::SetsAction, GlobalAction};

pub struct SetsScreenModel {
    loading: bool,
    sets: Vec<usize>,
}

impl SetsScreenModel {
    pub fn new() -> Self {
        Self {
            loading: false,
            sets: vec![],
        }
    }
}

pub fn update(
    action: &GlobalAction,
    model: &mut SetsScreenModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Sets(SetsAction::GetSets(_)) => {
            model.loading = true;
        }

        GlobalAction::Sets(SetsAction::GetSetsFailed(_)) => {
            model.loading = false;
        }

        GlobalAction::Sets(SetsAction::GetSetsSuccess(payload)) => {
            model.loading = false;
            model.sets = payload.sets.iter().map(|s| s.id).collect();
        }

        GlobalAction::Sets(SetsAction::DeleteSetSuccess(payload)) => {
            model.sets.retain(|&id| id != payload.set_id);
        }

        _ => {}
    }
}
