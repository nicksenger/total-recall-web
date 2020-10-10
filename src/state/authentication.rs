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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::{authentication::*, ErrorPayload};

    #[test]
    fn login_attempt() {
        let mut model = AuthenticationModel::new();
        update(
            &Msg::Authentication(AuthMsg::AttemptLogin(AttemptLoginPayload {
                username: "foo".to_owned(),
                password: "bar".to_owned(),
            })),
            &mut model,
        );

        assert_eq!(model.loading, true);
    }

    #[test]
    fn login_failure() {
        let mut model = AuthenticationModel {
            loading: true,
            username: Some("foo".to_owned()),
            token: Some("abc123".to_owned()),
        };
        update(
            &Msg::Authentication(AuthMsg::LoginFailed(ErrorPayload {
                content: "Failed!".to_owned(),
            })),
            &mut model,
        );

        assert_eq!(model.loading, false);
        assert_eq!(model.username, None);
        assert_eq!(model.token, None);
    }

    #[test]
    fn login_success() {
        let mut model = AuthenticationModel {
            loading: true,
            username: None,
            token: None,
        };
        update(
            &Msg::Authentication(AuthMsg::LoginSuccess(LoginSuccessPayload {
                username: "foo".to_owned(),
                token: "abc123".to_owned(),
            })),
            &mut model,
        );

        assert_eq!(model.loading, false);
        assert_eq!(model.username, Some("foo".to_owned()));
        assert_eq!(model.token, Some("abc123".to_owned()));
    }

    #[test]
    fn logout() {
        let mut model = AuthenticationModel {
            loading: false,
            username: Some("foo".to_owned()),
            token: Some("abc123".to_owned()),
        };
        update(&Msg::Authentication(AuthMsg::Logout), &mut model);

        assert_eq!(model.loading, false);
        assert_eq!(model.username, None);
        assert_eq!(model.token, None);
    }
}
