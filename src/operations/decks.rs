use graphql_client::{GraphQLQuery, Response as GQLResponse};
use seed::prelude::Orders;

use super::send_graphql_request;
use crate::{
    messages::{
        decks::{
            DecksMsg, DeleteDeckSuccessPayload, GetDecksSuccessPayload, GetLanguagesSuccessPayload,
        },
        ErrorPayload, Msg,
    },
    state::{
        entities::{Deck, Language},
        Model,
    },
    utilities::gql::get_gql_error_message,
};

type BigInt = u128;

macro_rules! generate_query {
    ($query:ident) => {
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = "src/operations/schema.json",
            query_path = "src/operations/decks.graphql",
            response_derives = "Debug"
        )]
        pub struct $query;
    };
}
generate_query!(LanguageList);
generate_query!(UserDecks);
generate_query!(CreateDeck);
generate_query!(DeleteDeck);

pub fn operate(msg: &Msg, _model: &Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Decks(DecksMsg::GetDecks(payload)) => {
            let username = payload.username.clone();
            orders.perform_cmd(async move {
                Msg::Decks(DecksMsg::GetDecksFetched(
                    username.clone(),
                    send_graphql_request(&UserDecks::build_query(user_decks::Variables {
                        username,
                    }))
                    .await,
                ))
            });
        }

        Msg::Decks(DecksMsg::GetDecksFetched(
            username,
            Ok(GQLResponse {
                data: Some(data),
                errors: None,
            }),
        )) => {
            orders.send_msg(Msg::Decks(DecksMsg::GetDecksSuccess(
                GetDecksSuccessPayload {
                    decks: data
                        .decks
                        .iter()
                        .map(|d| Deck {
                            id: d.id as usize,
                            language: d.language.name.clone(),
                            created: 0,
                            name: d.name.clone(),
                            owner: username.to_owned(),
                        })
                        .collect(),
                },
            )));
        }

        Msg::Decks(DecksMsg::GetDecksFetched(_, x)) => {
            orders.send_msg(Msg::Decks(DecksMsg::GetDecksFailed(ErrorPayload {
                content: get_gql_error_message(x, "Failed to retrieve decks."),
            })));
        }

        Msg::Decks(DecksMsg::DeleteDeck(payload)) => {
            let deck_id = payload.deck_id;
            orders.perform_cmd(async move {
                Msg::Decks(DecksMsg::DeleteDeckFetched(
                    deck_id,
                    send_graphql_request(&DeleteDeck::build_query(delete_deck::Variables {
                        id: deck_id as i64,
                    }))
                    .await,
                ))
            });
        }

        Msg::Decks(DecksMsg::DeleteDeckFetched(
            deck_id,
            Ok(GQLResponse {
                data: Some(_),
                errors: None,
            }),
        )) => {
            orders.send_msg(Msg::Decks(DecksMsg::DeleteDeckSuccess(
                DeleteDeckSuccessPayload { deck_id: *deck_id },
            )));
        }

        Msg::Decks(DecksMsg::DeleteDeckFetched(_, x)) => {
            orders.send_msg(Msg::Decks(DecksMsg::DeleteDeckFailed(ErrorPayload {
                content: get_gql_error_message(x, "Failed to delete deck."),
            })));
        }

        Msg::Decks(DecksMsg::GetLanguages) => {
            orders.perform_cmd(async move {
                Msg::Decks(DecksMsg::GetLanguagesFetched(
                    send_graphql_request(&LanguageList::build_query(language_list::Variables {}))
                        .await,
                ))
            });
        }

        Msg::Decks(DecksMsg::GetLanguagesFetched(Ok(GQLResponse {
            data: Some(data),
            errors: None,
        }))) => {
            orders.send_msg(Msg::Decks(DecksMsg::GetLanguagesSuccess(
                GetLanguagesSuccessPayload {
                    languages: data
                        .languages
                        .iter()
                        .map(|l| Language {
                            id: l.id as usize,
                            abbreviation: l.abbreviation.clone(),
                            name: l.name.clone(),
                        })
                        .collect(),
                },
            )));
        }

        Msg::Decks(DecksMsg::GetLanguagesFetched(x)) => {
            orders.send_msg(Msg::Decks(DecksMsg::GetLanguagesFailed(ErrorPayload {
                content: get_gql_error_message(x, "Failed to retrieve languages."),
            })));
        }

        _ => {}
    }
}
