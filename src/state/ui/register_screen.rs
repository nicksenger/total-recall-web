use crate::messages::{authentication::AuthMsg, Msg};

pub struct RegisterScreenModel {
    pub loading: bool,
}

impl RegisterScreenModel {
    pub fn new() -> Self {
        Self { loading: false }
    }
}

pub fn update(action: &Msg, model: &mut RegisterScreenModel) {
    match action {
        Msg::Authentication(AuthMsg::Register(_)) => {
            model.loading = true;
        }

        Msg::Authentication(AuthMsg::RegistrationFailed(_))
        | Msg::Authentication(AuthMsg::RegistrationSuccess) => {
            model.loading = false;
        }

        _ => {}
    }
}
