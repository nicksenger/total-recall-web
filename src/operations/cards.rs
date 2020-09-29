use graphql_client::{GraphQLQuery, Response as GQLResponse};
use seed::prelude::Orders;

use crate::{
    messages::{
        cards::{
            AddCardSuccessPayload, CardsMsg, DeleteCardSuccessPayload, EditCardLinkSuccessPayload,
            GetCardsSuccessPayload,
        },
        session::ScoreValue,
        ErrorPayload, Msg,
    },
    state::{entities::Card, Model},
    utilities::gql::{get_gql_error_message, send_graphql_request},
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

pub fn operate(msg: &Msg, model: &Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Cards(CardsMsg::AddCard(payload)) => {
            let token = model.authentication.token.clone();
            let deck_id = payload.deck_id;
            let back = payload.back.clone();
            let front = payload.front.clone();
            let link = payload.link.clone();
            orders.perform_cmd(async move {
                Msg::Cards(CardsMsg::AddCardFetched(
                    deck_id,
                    send_graphql_request(&CreateCard::build_query(create_card::Variables {
                        deck_id: deck_id as i64,
                        back,
                        front,
                        link,
                    }), token)
                    .await,
                ))
            });
        }

        Msg::Cards(CardsMsg::AddCardFetched(
            deck_id,
            Ok(GQLResponse {
                data: Some(_),
                errors: None,
            }),
        )) => {
            orders.send_msg(Msg::Cards(CardsMsg::AddCardSuccess(
                AddCardSuccessPayload { deck_id: *deck_id },
            )));
        }

        Msg::Cards(CardsMsg::AddCardFetched(_, x)) => {
            orders.send_msg(Msg::Cards(CardsMsg::AddCardFailed(ErrorPayload {
                content: get_gql_error_message(x, "Failed to add card."),
            })));
        }

        Msg::Cards(CardsMsg::GetCards(payload)) => {
            let token = model.authentication.token.clone();
            let deck_id = payload.deck_id;
            orders.perform_cmd(async move {
                Msg::Cards(CardsMsg::GetCardsFetched(
                    deck_id,
                    send_graphql_request(&DeckCards::build_query(deck_cards::Variables {
                        deck_id: deck_id as i64,
                    }), token)
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

        Msg::Cards(CardsMsg::EditCardLink(payload)) => {
            let token = model.authentication.token.clone();
            let card_id = payload.card_id;
            let link = payload.link.clone();
            orders.perform_cmd(async move {
                Msg::Cards(CardsMsg::EditCardLinkFetched(
                    card_id,
                    link.clone(),
                    send_graphql_request(&EditCardLink::build_query(edit_card_link::Variables {
                        card_id: card_id as i64,
                        link,
                    }), token)
                    .await,
                ))
            });
        }

        Msg::Cards(CardsMsg::EditCardLinkFetched(
            card_id,
            link,
            Ok(GQLResponse {
                data: Some(_),
                errors: None,
            }),
        )) => {
            orders.send_msg(Msg::Cards(CardsMsg::EditCardLinkSuccess(
                EditCardLinkSuccessPayload {
                    card_id: *card_id,
                    link: link.to_owned(),
                },
            )));
        }

        Msg::Cards(CardsMsg::EditCardLinkFetched(_, _, x)) => {
            orders.send_msg(Msg::Cards(CardsMsg::EditCardLinkFailed(ErrorPayload {
                content: get_gql_error_message(x, "Failed to edit card link."),
            })));
        }

        Msg::Cards(CardsMsg::DeleteCard(payload)) => {
            let token = model.authentication.token.clone();
            let card_id = payload.card_id;
            orders.perform_cmd(async move {
                Msg::Cards(CardsMsg::DeleteCardFetched(
                    card_id,
                    send_graphql_request(&DeleteCard::build_query(delete_card::Variables {
                        card_id: card_id as i64,
                    }), token)
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
