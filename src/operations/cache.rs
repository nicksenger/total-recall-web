use seed::prelude::*;

use crate::{
    messages::{
        authentication::AuthMsg, cache::CacheMsg, cards::CardsMsg, session::SessionMsg, Msg,
    },
    state::Model,
};

pub fn operate(msg: &Msg, model: &Model, orders: &mut impl Orders<Msg>) {
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

        Msg::Cache(CacheMsg::ToggleDarkTheme(dark)) => {
            let _ = LocalStorage::insert("dark_theme", dark);
            if let Ok(Some(element)) = seed::document().query_selector("html") {
                let _ = element.set_attribute(
                    "class",
                    if *dark {
                        "spectrum spectrum--dark spectrum--medium"
                    } else {
                        "spectrum spectrum--light spectrum--medium"
                    },
                );
            }
        }

        Msg::Cache(CacheMsg::Hydrate) => {
            orders.send_msg(Msg::Cache(CacheMsg::ToggleDarkTheme(
                match LocalStorage::get("dark_theme") {
                    Ok(x) => x,
                    _ => match seed::window().match_media("(prefers-color-scheme: dark)") {
                        Ok(Some(list)) => {
                            if list.matches() {
                                true
                            } else {
                                false
                            }
                        }
                        _ => false,
                    },
                },
            )));
        }

        _ => {}
    }
}
