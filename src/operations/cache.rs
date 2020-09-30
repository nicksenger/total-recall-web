use seed::prelude::*;

use crate::{
    messages::{authentication::AuthMsg, cards::CardsMsg, session::SessionMsg, Msg},
    state::Model,
};

pub fn operate(msg: &Msg, model: &Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Authentication(AuthMsg::LoginSuccess(payload)) => {
            let _ = LocalStorage::insert("username", &payload.username);
            let _ = LocalStorage::insert("token", &payload.token);
        }

        Msg::Cards(CardsMsg::GetCardsSuccess(_)) => {
            let _ = LocalStorage::insert("cards", &model.entities.cards);
        }

        Msg::Session(SessionMsg::RateCard(_))
        | Msg::Session(SessionMsg::ReviewCard(_))
        | Msg::Session(SessionMsg::Study(_)) => {
            let _ = LocalStorage::insert("rate_queue", &model.session.rate_queue);
            let _ = LocalStorage::insert("review_queue", &model.session.review_queue);
            let _ = LocalStorage::insert("status", &model.session.status);
        }

        Msg::Session(SessionMsg::RevealCard(_)) => {
            let _ = LocalStorage::insert("status", &model.session.status);
        }

        Msg::Authentication(AuthMsg::Logout) => {
            let _ = LocalStorage::remove("username");
            let _ = LocalStorage::remove("token");
            let _ = LocalStorage::remove("cards");
            let _ = LocalStorage::remove("rate_queue");
            let _ = LocalStorage::remove("review_queue");
            let _ = LocalStorage::remove("status");
        }

        _ => {}
    }
}
