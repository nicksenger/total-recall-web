use graphql_client::GraphQLQuery;
use seed::prelude::Orders;

use crate::{messages::Msg, state::Model};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/session.graphql"
)]
pub struct RateCard;

pub fn operate(msg: &Msg, model: &Model, orders: &mut impl Orders<Msg>) {}
