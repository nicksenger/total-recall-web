use graphql_client::Response;
use seed::prelude::fetch;
use serde::{Deserialize, Serialize};

use super::ErrorPayload;
use crate::operations::authentication;

pub struct AttemptLoginPayload {
    pub username: String,
    pub password: String,
}

pub struct LoginSuccessPayload {
    pub username: String,
    pub token: String,
}

pub struct RegisterPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct AuthenticationRequestBody {
    pub password: String,
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct AuthenticationResponseBody {
    pub token: String,
}

pub enum AuthMsg {
    AttemptLogin(AttemptLoginPayload),
    AttemptLoginFetched(String, fetch::Result<AuthenticationResponseBody>),
    LoginFailed(ErrorPayload),
    LoginSuccess(LoginSuccessPayload),
    Logout,
    Register(RegisterPayload),
    RegistrationFetched(fetch::Result<Response<authentication::register::ResponseData>>),
    RegistrationFailed(ErrorPayload),
    RegistrationSuccess,
}
