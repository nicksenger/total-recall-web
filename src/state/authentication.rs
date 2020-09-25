use seed::prelude::Orders;

use crate::actions::{authentication::AuthAction, cache::CacheAction, GlobalAction};

pub struct AuthenticationModel {
    loading: bool,
    username: Option<String>,
    token: Option<String>,
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
    action: &GlobalAction,
    model: &mut AuthenticationModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Authentication(AuthAction::AttemptLogin(_)) => {
            model.loading = true;
        }

        GlobalAction::Authentication(AuthAction::LoginFailed(_)) => {
            model.loading = false;
            model.token = None;
            model.username = None;
        }

        GlobalAction::Authentication(AuthAction::LoginSuccess(payload)) => {
            model.loading = false;
            model.token = Some(payload.token.clone());
            model.username = Some(payload.username.clone());
        }

        GlobalAction::Cache(CacheAction::HydrateCache(payload)) => {
            if let Some((token, username)) = &payload.auth {
                model.username = Some(username.clone());
                model.token = Some(token.clone());
            }
        }

        _ => {}
    }
}
