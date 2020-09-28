use seed::prelude::*;

use crate::{
    messages::{
        authentication::AuthMsg,
        cards::{CardsMsg, GetCardsPayload},
        decks::{DecksMsg, GetDecksPayload},
        routing::RoutingMessage,
        sets::{GetSetsPayload, SetsMsg},
        Msg,
    },
    state::{routing::Route, Model},
};

pub fn operate(msg: &Msg, _model: &Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Routing(RoutingMessage::Push(r)) => {
            Url::from(r).go_and_push();
            orders.send_msg(Msg::Routing(RoutingMessage::Navigate(r.clone())));
        }

        Msg::Authentication(AuthMsg::LoginSuccess(payload)) => {
            orders.send_msg(Msg::Routing(RoutingMessage::Push(Route::Decks(
                payload.username.clone(),
            ))));
        }

        Msg::Authentication(AuthMsg::RegistrationSuccess) => {
            orders.send_msg(Msg::Routing(RoutingMessage::Push(Route::Login)));
        }

        Msg::Routing(RoutingMessage::Navigate(Route::Decks(username))) => {
            orders.send_msg(Msg::Decks(DecksMsg::GetDecks(GetDecksPayload {
                username: username.clone(),
            })));
        }

        Msg::Routing(RoutingMessage::Navigate(Route::DeckCards(_, deck_id))) => {
            orders.send_msg(Msg::Cards(CardsMsg::GetCards(GetCardsPayload {
                deck_id: *deck_id,
            })));
        }

        Msg::Routing(RoutingMessage::Navigate(Route::DeckSets(_, deck_id))) => {
            orders.send_msg(Msg::Sets(SetsMsg::GetSets(GetSetsPayload {
                deck_id: *deck_id,
            })));
        }

        _ => {}
    }
}
