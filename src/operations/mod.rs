use seed::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{messages::Msg, state::Model};

pub mod authentication;
pub mod cards;
pub mod decks;
pub mod session;
pub mod sets;

const API_URL: &str = "https://trc.nsenger.com/graphql/";

pub fn operate(msg: &Msg, model: &Model, orders: &mut impl Orders<Msg>) {
    authentication::operate(msg, model, orders);
    cards::operate(msg, model, orders);
    decks::operate(msg, model, orders);
    session::operate(msg, model, orders);
    sets::operate(msg, model, orders);
}

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
