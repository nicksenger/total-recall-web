use seed::prelude::*;
use serde::{Deserialize, Serialize};

pub mod authentication;
pub mod cards;
pub mod decks;
pub mod session;
pub mod sets;

const API_URL: &str = "https://trc.nsenger.com/graphql/";

pub async fn send_graphql_request<V, T>(variables: &V) -> fetch::Result<T>
where
    V: Serialize,
    T: for<'de> Deserialize<'de> + 'static,
{
    Request::new(API_URL)
        .method(Method::Post)
        .json(variables)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}
