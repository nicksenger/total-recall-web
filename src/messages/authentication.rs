use super::ErrorPayload;

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

pub enum AuthMsg {
    AttemptLogin(AttemptLoginPayload),
    LoginFailed(ErrorPayload),
    LoginSuccess(LoginSuccessPayload),
    Logout,
    Register(RegisterPayload),
    RegistrationFailed(ErrorPayload),
    RegistrationSuccess,
    RetrieveAuthInfo,
}
