use graphql_client::{GraphQLQuery, Response as GQLResponse};
use seed::prelude::Orders;

use crate::{
    messages::{
        sets::{AddSetSuccessPayload, DeleteSetSuccessPayload, GetSetsSuccessPayload, SetsMsg},
        ErrorPayload, Msg,
    },
    state::{entities::Set, Model},
    utilities::gql::{get_gql_error_message, send_graphql_request},
};

type BigInt = u128;

macro_rules! generate_query {
    ($query:ident) => {
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = "src/operations/schema.json",
            query_path = "src/operations/sets.graphql",
            response_derives = "Debug"
        )]
        pub struct $query;
    };
}
generate_query!(UserSets);
generate_query!(CreateSet);
generate_query!(DeleteSet);

pub fn operate(msg: &Msg, model: &Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Sets(SetsMsg::AddSet(payload)) => {
            let token = model.authentication.token.clone();
            let deck_id = payload.deck_id;
            let card_ids = payload
                .card_ids
                .iter()
                .map(|id| *id as i64)
                .collect::<Vec<i64>>();
            let name = payload.name.to_owned();
            orders.perform_cmd(async move {
                Msg::Sets(SetsMsg::AddSetFetched(
                    deck_id,
                    send_graphql_request(&CreateSet::build_query(create_set::Variables {
                        deck_id: deck_id as i64,
                        card_ids,
                        name,
                    }), token)
                    .await,
                ))
            });
        }

        Msg::Sets(SetsMsg::AddSetFetched(
            deck_id,
            Ok(GQLResponse {
                data: Some(_),
                errors: None,
            }),
        )) => {
            orders.send_msg(Msg::Sets(SetsMsg::AddSetSuccess(AddSetSuccessPayload {
                deck_id: *deck_id,
            })));
        }

        Msg::Sets(SetsMsg::AddSetFetched(_, x)) => {
            orders.send_msg(Msg::Sets(SetsMsg::AddSetFailed(ErrorPayload {
                content: get_gql_error_message(x, "Failed to create set."),
            })));
        }

        Msg::Sets(SetsMsg::GetSets(payload)) => {
            let token = model.authentication.token.clone();
            let deck_id = payload.deck_id;
            orders.perform_cmd(async move {
                Msg::Sets(SetsMsg::GetSetsFetched(
                    deck_id,
                    send_graphql_request(&UserSets::build_query(user_sets::Variables {
                        deck_id: deck_id as i64,
                    }), token)
                    .await,
                ))
            });
        }

        Msg::Sets(SetsMsg::GetSetsFetched(
            deck_id,
            Ok(GQLResponse {
                data: Some(data),
                errors: None,
            }),
        )) => {
            orders.send_msg(Msg::Sets(SetsMsg::GetSetsSuccess(GetSetsSuccessPayload {
                deck_id: *deck_id,
                sets: data
                    .sets
                    .iter()
                    .map(|s| Set {
                        id: s.id as usize,
                        card_ids: s
                            .cards
                            .iter()
                            .map(|c| c.card_id.id as usize)
                            .collect::<Vec<usize>>(),
                        deck: *deck_id,
                        name: s.name.clone(),
                        owner: s.owner.username.clone(),
                    })
                    .collect(),
            })));
        }

        Msg::Sets(SetsMsg::GetSetsFetched(_, x)) => {
            orders.send_msg(Msg::Sets(SetsMsg::GetSetsFailed(ErrorPayload {
                content: get_gql_error_message(x, "Failed to retrieve sets."),
            })));
        }

        Msg::Sets(SetsMsg::DeleteSet(payload)) => {
            let token = model.authentication.token.clone();
            let set_id = payload.set_id;
            orders.perform_cmd(async move {
                Msg::Sets(SetsMsg::DeleteSetFetched(
                    set_id,
                    send_graphql_request(&DeleteSet::build_query(delete_set::Variables {
                        set_id: set_id as i64,
                    }), token)
                    .await,
                ))
            });
        }

        Msg::Sets(SetsMsg::DeleteSetFetched(
            set_id,
            Ok(GQLResponse {
                data: Some(_),
                errors: None,
            }),
        )) => {
            orders.send_msg(Msg::Sets(SetsMsg::DeleteSetSuccess(
                DeleteSetSuccessPayload { set_id: *set_id },
            )));
        }

        Msg::Sets(SetsMsg::DeleteSetFetched(_, x)) => {
            orders.send_msg(Msg::Sets(SetsMsg::DeleteSetFailed(ErrorPayload {
                content: get_gql_error_message(x, "Failed to delete set."),
            })));
        }

        _ => {}
    }
}
