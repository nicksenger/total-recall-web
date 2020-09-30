use seed::{browser::web_storage::WebStorageError, prelude::*};

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

        Msg::Cache(CacheMsg::Hydrate) => {
            let username_result: Result<String, WebStorageError> = LocalStorage::get("username");
            if let Ok(username) = username_result {
                model.username = Some(username);
            }

            let token_result: Result<String, WebStorageError> = LocalStorage::get("token");
            if let Ok(token) = token_result {
                model.token = Some(token);
            }
        }

        _ => {}
    }
}
