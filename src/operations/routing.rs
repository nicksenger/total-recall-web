use seed::prelude::*;

use crate::{
    messages::{
        authentication::AuthMsg,
        cards::{AddCardSuccessPayload, CardsMsg, GetCardsPayload},
        decks::{DecksMsg, GetDecksPayload},
        routing::RoutingMsg,
        session::SessionMsg,
        sets::{AddSetSuccessPayload, GetSetsPayload, GotoAddSetPayload, SetsMsg},
        Msg,
    },
    state::{routing::Route, Model},
};

pub fn operate(msg: &Msg, model: &Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Routing(RoutingMsg::Push(r)) => {
            Url::from(r).go_and_push();
            orders.send_msg(Msg::Routing(RoutingMsg::Navigate(r.clone())));
        }

        Msg::Authentication(AuthMsg::LoginSuccess(payload)) => {
            orders.send_msg(Msg::Routing(RoutingMsg::Push(Route::Decks(
                payload.username.clone(),
            ))));
        }

        Msg::Authentication(AuthMsg::RegistrationSuccess) => {
            orders.send_msg(Msg::Routing(RoutingMsg::Push(Route::Login)));
        }

        Msg::Decks(DecksMsg::AddDeckSuccess(_)) => {
            if let Some(username) = &model.authentication.username {
                orders.send_msg(Msg::Routing(RoutingMsg::Push(Route::Decks(
                    username.to_owned(),
                ))));
            }
        }

        Msg::Decks(DecksMsg::DeleteDeckSuccess(_)) => {
            if let Some(username) = &model.authentication.username {
                orders.send_msg(Msg::Routing(RoutingMsg::Push(Route::Decks(
                    username.to_owned(),
                ))));
            }
        }

        Msg::Sets(SetsMsg::GotoAddSet(GotoAddSetPayload {
            username,
            deck_id,
            cards: _,
        })) => {
            orders.send_msg(Msg::Routing(RoutingMsg::Push(Route::AddSet(
                username.to_owned(),
                *deck_id,
            ))));
        }

        Msg::Sets(SetsMsg::AddSetSuccess(AddSetSuccessPayload { deck_id })) => {
            if let Some(username) = &model.authentication.username {
                orders.send_msg(Msg::Routing(RoutingMsg::Push(Route::DeckSets(
                    username.to_owned(),
                    *deck_id,
                ))));
            }
        }

        Msg::Cards(CardsMsg::AddCardSuccess(AddCardSuccessPayload { deck_id })) => {
            if let Some(username) = &model.authentication.username {
                orders.send_msg(Msg::Routing(RoutingMsg::Push(Route::DeckCards(
                    username.to_owned(),
                    *deck_id,
                ))));
            }
        }

        Msg::Cards(CardsMsg::DeleteCardSuccess(_)) => {
            let _ = seed::history().back();
        }

        Msg::Sets(SetsMsg::DeleteSetSuccess(_)) => {
            let _ = seed::history().back();
        }

        Msg::Session(SessionMsg::Study(_)) => {
            orders.send_msg(Msg::Routing(RoutingMsg::Push(Route::Study)));
        }

        Msg::Routing(RoutingMsg::Navigate(x)) => {
            orders.send_msg(Msg::Routing(RoutingMsg::ModalOpen(false)));

            match x {
                Route::Decks(username) => {
                    orders.send_msg(Msg::Decks(DecksMsg::GetDecks(GetDecksPayload {
                        username: username.clone(),
                    })));
                }

                Route::DeckCards(_, deck_id) => {
                    orders.send_msg(Msg::Cards(CardsMsg::GetCards(GetCardsPayload {
                        deck_id: *deck_id,
                    })));
                }

                Route::DeckSets(_, deck_id) => {
                    orders.send_msg(Msg::Sets(SetsMsg::GetSets(GetSetsPayload {
                        deck_id: *deck_id,
                    })));
                }

                Route::AddDeck(_) => {
                    orders.send_msg(Msg::Decks(DecksMsg::GetLanguages));
                },

                _ => {}
            }
        }

        _ => {}
    }
}
