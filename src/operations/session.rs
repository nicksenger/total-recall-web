use graphql_client::{GraphQLQuery, Response as GQLResponse};
use seed::prelude::Orders;

use crate::{
    messages::{
        session::{RateCardSuccessPayload, ScoreValue, SessionMsg},
        ErrorPayload, Msg,
    },
    operations::session,
    state::Model,
    utilities::gql::{get_gql_error_message, send_graphql_request},
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/session.graphql"
)]
pub struct RateCard;

pub fn operate(msg: &Msg, model: &Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Session(SessionMsg::RateCard(payload)) => {
            let token = model.authentication.token.clone();
            let card_id = payload.card_id;
            let score = payload.rating.clone();
            orders.perform_cmd(async move {
                Msg::Session(SessionMsg::RateCardFetched(
                    card_id,
                    score.clone(),
                    send_graphql_request(&RateCard::build_query(rate_card::Variables {
                        card_id: card_id as i64,
                        rating: match score {
                            ScoreValue::Zero => session::rate_card::ScoreValue::ZERO,
                            ScoreValue::One => session::rate_card::ScoreValue::ONE,
                            ScoreValue::Two => session::rate_card::ScoreValue::TWO,
                            ScoreValue::Three => session::rate_card::ScoreValue::THREE,
                            ScoreValue::Four => session::rate_card::ScoreValue::FOUR,
                            ScoreValue::Five => session::rate_card::ScoreValue::FIVE,
                        },
                    }), token)
                    .await,
                ))
            });
        }

        Msg::Session(SessionMsg::RateCardFetched(
            card_id,
            score,
            Ok(GQLResponse {
                data: Some(_),
                errors: None,
            }),
        )) => {
            orders.send_msg(Msg::Session(SessionMsg::RateCardSuccess(
                RateCardSuccessPayload {
                    card_id: *card_id,
                    rating: score.clone(),
                },
            )));
        }

        Msg::Session(SessionMsg::RateCardFetched(_, _, x)) => {
            orders.send_msg(Msg::Session(SessionMsg::RateCardFailed(ErrorPayload {
                content: get_gql_error_message(x, "Failed to score card."),
            })));
        }

        _ => {}
    }
}
