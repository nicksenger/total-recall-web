use crate::messages::{authentication::AuthMsg, Msg};

pub struct AuthenticationModel {
    pub loading: bool,
    pub username: Option<String>,
    pub token: Option<String>,
}

impl AuthenticationModel {
    pub fn new() -> Self {
        Self {
            loading: false,
            username: None,
            token: None,
        }
    }
}

pub fn update(action: &Msg, model: &mut AuthenticationModel) {
    match action {
        Msg::Authentication(AuthMsg::AttemptLogin(_)) => {
            model.loading = true;
        }

        Msg::Authentication(AuthMsg::LoginFailed(_)) => {
            model.loading = false;
            model.token = None;
            model.username = None;
        }

        Msg::Authentication(AuthMsg::LoginSuccess(payload)) => {
            model.loading = false;
            model.token = Some(payload.token.clone());
            model.username = Some(payload.username.clone());
        }

        Msg::Authentication(AuthMsg::Logout) => {
            model.token = None;
            model.username = None;
        }

        _ => {}
    }
}
