use graphql_client::{GraphQLQuery, Response as GQLResponse};
use seed::prelude::Orders;

use crate::{
    messages::{
        session::{PlayAudioPayload, RateCardSuccessPayload, ScoreValue, SessionMsg},
        ErrorPayload, Msg,
    },
    operations::session,
    state::Model,
    utilities::gql::{get_gql_error_message, send_graphql_request},
    BASE_URI,
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
                    send_graphql_request(
                        &RateCard::build_query(rate_card::Variables {
                            card_id: card_id as i64,
                            rating: match score {
                                ScoreValue::Zero => session::rate_card::ScoreValue::ZERO,
                                ScoreValue::One => session::rate_card::ScoreValue::ONE,
                                ScoreValue::Two => session::rate_card::ScoreValue::TWO,
                                ScoreValue::Three => session::rate_card::ScoreValue::THREE,
                                ScoreValue::Four => session::rate_card::ScoreValue::FOUR,
                                ScoreValue::Five => session::rate_card::ScoreValue::FIVE,
                            },
                        }),
                        token,
                    )
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

        Msg::Session(SessionMsg::PlayAudio(payload)) => {
            if let Ok(element) = seed::document().create_element("audio") {
                let _ = element.set_attribute("autoplay", "autoplay");
                let _ = element.set_attribute("allow", "autoplay");
                let _ = element.set_attribute(
                    "src",
                    format!("{}/{}", BASE_URI, payload.uri.as_str()).as_str(),
                );
            }
        }

        Msg::Session(SessionMsg::RevealCard(payload)) => {
            let uri = payload.card.audio.clone();
            orders.send_msg(Msg::Session(SessionMsg::PlayAudio(PlayAudioPayload {
                uri,
            })));
        }

        _ => {}
    }
}
