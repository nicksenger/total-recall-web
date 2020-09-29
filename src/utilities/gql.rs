use graphql_client::Response as GQLResponse;
use seed::prelude::*;
use serde::{Deserialize, Serialize};

use crate::BASE_URI;

pub async fn send_graphql_request<V, T>(variables: &V, token: Option<String>) -> fetch::Result<T>
where
    V: Serialize,
    T: for<'de> Deserialize<'de> + 'static,
{
    Request::new(format!("{}/graphql", BASE_URI))
        .method(Method::Post)
        .header(Header::authorization(token.unwrap_or("".to_owned())))
        .json(variables)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub fn get_gql_error_message<T>(
    response: &fetch::Result<GQLResponse<T>>,
    generic_message: &str,
) -> String {
    match response {
        Ok(GQLResponse {
            data: _,
            errors: Some(errors),
        }) => errors
            .last()
            .map(|e| e.message.to_owned())
            .unwrap_or(generic_message.to_string()),
        _ => generic_message.to_string(),
    }
}
