use seed::prelude::Orders;

use crate::actions::{sets::SetsAction, GlobalAction};
use crate::state::entities::Set;

pub struct SetDetailsScreenModel {
    loading: bool,
    selected_set: Option<Set>,
}

impl SetDetailsScreenModel {
    pub fn new() -> Self {
        Self {
            loading: false,
            selected_set: None,
        }
    }
}

pub fn update(
    action: &GlobalAction,
    model: &mut SetDetailsScreenModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Sets(SetsAction::DeleteSet(_)) => {
            model.loading = true;
        }

        GlobalAction::Sets(SetsAction::DeleteSetFailed(_))
        | GlobalAction::Sets(SetsAction::DeleteSetSuccess(_)) => {
            model.loading = false;
        }

        GlobalAction::Sets(SetsAction::ViewSetDetails(payload)) => {
            model.selected_set = Some(payload.set.clone());
        }

        _ => {}
    }
}
