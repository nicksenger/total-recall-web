use seed::prelude::Orders;

use crate::actions::{authentication::AuthAction, GlobalAction};

pub struct RegisterScreenModel {
    loading: bool,
}

impl RegisterScreenModel {
    pub fn new() -> Self {
        Self { loading: false }
    }
}

pub fn update(
    action: &GlobalAction,
    model: &mut RegisterScreenModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Authentication(AuthAction::Register(_)) => {
            model.loading = true;
        }

        GlobalAction::Authentication(AuthAction::RegistrationFailed(_))
        | GlobalAction::Authentication(AuthAction::RegistrationSuccess) => {
            model.loading = false;
        }

        _ => {}
    }
}
