use graphql_client::{GraphQLQuery, Response as GQLResponse};
use seed::prelude::*;

use super::send_graphql_request;
use crate::{
    messages::{
        authentication::{
            AuthMsg, AuthenticationRequestBody, AuthenticationResponseBody, LoginSuccessPayload,
        },
        ErrorPayload, Msg,
    },
    state::Model,
    utilities::gql::get_gql_error_message,
    BASE_URI,
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/authentication.graphql"
)]
pub struct Register;

async fn authenticate(
    username: String,
    password: String,
) -> fetch::Result<AuthenticationResponseBody> {
    Request::new(format!("{}/login", BASE_URI))
        .method(Method::Post)
        .json(&AuthenticationRequestBody { username, password })?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub fn operate(msg: &Msg, _model: &Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Authentication(AuthMsg::AttemptLogin(payload)) => {
            let username = payload.username.clone();
            let password = payload.password.clone();
            orders.perform_cmd(async move {
                Msg::Authentication(AuthMsg::AttemptLoginFetched(
                    username.clone(),
                    authenticate(username, password).await,
                ))
            });
        }

        Msg::Authentication(AuthMsg::AttemptLoginFetched(username, Ok(response))) => {
            orders.send_msg(Msg::Authentication(AuthMsg::LoginSuccess(
                LoginSuccessPayload {
                    username: username.clone(),
                    token: response.token.clone(),
                },
            )));
        }

        Msg::Authentication(AuthMsg::AttemptLoginFetched(_, Err(_))) => {
            orders.send_msg(Msg::Authentication(AuthMsg::LoginFailed(ErrorPayload {
                content: "Login failed.".to_owned(),
            })));
        }

        Msg::Authentication(AuthMsg::Register(payload)) => {
            let username = payload.username.clone();
            let password = payload.username.clone();
            orders.perform_cmd(async move {
                Msg::Authentication(AuthMsg::RegistrationFetched(
                    send_graphql_request(&Register::build_query(register::Variables {
                        username,
                        password,
                    }))
                    .await,
                ))
            });
        }

        Msg::Authentication(AuthMsg::RegistrationFetched(Ok(GQLResponse {
            data: Some(_),
            errors: None,
        }))) => {
            orders.send_msg(Msg::Authentication(AuthMsg::RegistrationSuccess));
        }

        Msg::Authentication(AuthMsg::RegistrationFetched(x)) => {
            orders.send_msg(Msg::Authentication(AuthMsg::RegistrationFailed(
                ErrorPayload {
                    content: get_gql_error_message(x, "Registration failed."),
                },
            )));
        }

        _ => {}
    }
}
