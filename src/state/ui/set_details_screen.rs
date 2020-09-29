use crate::messages::{sets::SetsMsg, Msg};

pub struct SetDetailsScreenModel {
    pub loading: bool,
}

impl SetDetailsScreenModel {
    pub fn new() -> Self {
        Self {
            loading: false,
        }
    }
}

pub fn update(
    action: &Msg,
    model: &mut SetDetailsScreenModel,
) {
    match action {
        Msg::Sets(SetsMsg::DeleteSet(_)) => {
            model.loading = true;
        }

        Msg::Sets(SetsMsg::DeleteSetFailed(_))
        | Msg::Sets(SetsMsg::DeleteSetSuccess(_)) => {
            model.loading = false;
        }

        _ => {}
    }
}
