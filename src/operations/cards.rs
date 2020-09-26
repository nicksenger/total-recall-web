use graphql_client::{GraphQLQuery, Response as GQLResponse};
use seed::prelude::Orders;

use super::send_graphql_request;
use crate::{
    messages::{
        cards::{CardsMsg, DeleteCardSuccessPayload, GetCardsSuccessPayload},
        session::ScoreValue,
        ErrorPayload, Msg,
    },
    state::{entities::Card, Model},
    utilities::gql::get_gql_error_message,
};

type BigInt = u128;

macro_rules! generate_query {
    ($query:ident) => {
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = "src/operations/schema.json",
            query_path = "src/operations/cards.graphql",
            response_derives = "Debug"
        )]
        pub struct $query;
    };
}
generate_query!(DeckCards);
generate_query!(CreateCard);
generate_query!(DeleteCard);
generate_query!(EditCardLink);

pub fn operate(msg: &Msg, _model: &Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Cards(CardsMsg::GetCards(payload)) => {
            let deck_id = payload.deck_id;
            orders.perform_cmd(async move {
                Msg::Cards(CardsMsg::GetCardsFetched(
                    deck_id,
                    send_graphql_request(&DeckCards::build_query(deck_cards::Variables {
                        deck_id: deck_id as i64,
                    }))
                    .await,
                ))
            });
        }

        Msg::Cards(CardsMsg::GetCardsFetched(
            deck_id,
            Ok(GQLResponse {
                data: Some(data),
                errors: None,
            }),
        )) => {
            orders.send_msg(Msg::Cards(CardsMsg::GetCardsSuccess(
                GetCardsSuccessPayload {
                    deck_id: *deck_id,
                    cards: (&data)
                        .cards
                        .iter()
                        .map(|c| Card {
                            id: c.id as usize,
                            created: c.created_at,
                            last_seen: c.scores.last().map(|s| s.created_at).unwrap_or(0),
                            front: c.front.clone(),
                            back: c.back.text.clone(),
                            score: c
                                .scores
                                .iter()
                                .map(|s| match s.value {
                                    deck_cards::ScoreValue::ZERO => ScoreValue::Zero,
                                    deck_cards::ScoreValue::ONE => ScoreValue::One,
                                    deck_cards::ScoreValue::TWO => ScoreValue::Two,
                                    deck_cards::ScoreValue::THREE => ScoreValue::Three,
                                    deck_cards::ScoreValue::FOUR => ScoreValue::Four,
                                    deck_cards::ScoreValue::FIVE => ScoreValue::Five,
                                    _ => ScoreValue::Zero,
                                })
                                .collect::<Vec<ScoreValue>>(),
                            audio: c
                                .back
                                .audio
                                .as_ref()
                                .map(|s| s.to_owned())
                                .unwrap_or("".to_owned()),
                            image: c
                                .back
                                .image
                                .as_ref()
                                .map(|s| s.to_owned())
                                .unwrap_or("".to_owned()),
                            link: c.link.clone(),
                        })
                        .collect(),
                },
            )));
        }

        Msg::Cards(CardsMsg::GetCardsFetched(_, x)) => {
            orders.send_msg(Msg::Cards(CardsMsg::GetCardsFailed(ErrorPayload {
                content: get_gql_error_message(x, "Failed to retrieve cards."),
            })));
        }

        Msg::Cards(CardsMsg::DeleteCard(payload)) => {
            let card_id = payload.card_id;
            orders.perform_cmd(async move {
                Msg::Cards(CardsMsg::DeleteCardFetched(
                    card_id,
                    send_graphql_request(&DeleteCard::build_query(delete_card::Variables {
                        card_id: card_id as i64,
                    }))
                    .await,
                ))
            });
        }

        Msg::Cards(CardsMsg::DeleteCardFetched(
            card_id,
            Ok(GQLResponse {
                data: Some(_),
                errors: None,
            }),
        )) => {
            orders.send_msg(Msg::Cards(CardsMsg::DeleteCardSuccess(
                DeleteCardSuccessPayload { card_id: *card_id },
            )));
        }

        Msg::Cards(CardsMsg::DeleteCardFetched(_, x)) => {
            orders.send_msg(Msg::Cards(CardsMsg::DeleteCardFailed(ErrorPayload {
                content: get_gql_error_message(x, "Failed to delete card."),
            })));
        }

        _ => {}
    }
}
