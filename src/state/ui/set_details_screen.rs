use seed::prelude::Orders;

use crate::messages::{sets::SetsMsg, Msg};
use crate::state::entities::Set;

pub struct SetDetailsScreenModel {
    pub loading: bool,
    pub selected_set: Option<Set>,
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
    action: &Msg,
    model: &mut SetDetailsScreenModel,
    orders: &mut impl Orders<Msg>,
) {
    match action {
        Msg::Sets(SetsMsg::DeleteSet(_)) => {
            model.loading = true;
        }

        Msg::Sets(SetsMsg::DeleteSetFailed(_))
        | Msg::Sets(SetsMsg::DeleteSetSuccess(_)) => {
            model.loading = false;
        }

        Msg::Sets(SetsMsg::ViewSetDetails(payload)) => {
            model.selected_set = Some(payload.set.clone());
        }

        _ => {}
    }
}
