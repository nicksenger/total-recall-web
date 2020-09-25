use seed::prelude::Orders;

use crate::messages::{authentication::AuthMsg, cache::CacheMsg, Msg};

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

pub fn update(
    action: &Msg,
    model: &mut AuthenticationModel,
    orders: &mut impl Orders<Msg>,
) {
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

        Msg::Cache(CacheMsg::HydrateCache(payload)) => {
            if let Some((token, username)) = &payload.auth {
                model.username = Some(username.clone());
                model.token = Some(token.clone());
            }
        }

        _ => {}
    }
}
