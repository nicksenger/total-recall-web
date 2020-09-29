use std::{convert::TryFrom, fmt};

use seed::prelude::*;

use crate::messages::{routing::RoutingMessage, Msg};

#[derive(Clone, Debug)]
pub enum Route {
    Home,
    Login,
    Register,
    Decks(String),
    DeckCards(String, usize),
    DeckDetails(String, usize),
    DeckSets(String, usize),
    CardDetails(usize),
    SetDetails(usize),
    AddDeck(String),
    AddCard(String, usize),
    AddSet(String, usize),
    NotFound,
}

impl Route {
    pub fn path(&self) -> Vec<String> {
        match self {
            Route::Home => vec![],
            Route::Login => vec!["login".to_owned()],
            Route::Register => vec!["register".to_owned()],
            Route::Decks(username) => {
                vec!["user".to_owned(), username.to_owned(), "decks".to_owned()]
            }
            Route::DeckCards(username, deck_id) => vec![
                "user".to_owned(),
                username.to_owned(),
                "decks".to_owned(),
                format!("{}", deck_id),
                "cards".to_owned(),
            ],
            Route::DeckSets(username, deck_id) => vec![
                "user".to_owned(),
                username.to_owned(),
                "decks".to_owned(),
                format!("{}", deck_id),
                "sets".to_owned(),
            ],
            Route::CardDetails(card_id) => vec!["cards".to_owned(), format!("{}", card_id)],
            Route::SetDetails(set_id) => vec!["sets".to_owned(), format!("{}", set_id)],
            Route::AddDeck(username) => vec![
                "user".to_owned(),
                username.to_owned(),
                "add-deck".to_owned(),
            ],
            Route::DeckDetails(username, deck_id) => vec![
                "user".to_owned(),
                username.to_owned(),
                "decks".to_owned(),
                format!("{}", deck_id),
            ],
            Route::AddCard(username, deck_id) => vec![
                "user".to_owned(),
                username.to_owned(),
                "decks".to_owned(),
                format!("{}", deck_id),
                "add-card".to_owned(),
            ],
            Route::AddSet(username, deck_id) => vec![
                "user".to_owned(),
                username.to_owned(),
                "decks".to_owned(),
                format!("{}", deck_id),
                "add-set".to_owned(),
            ],
            Route::NotFound => vec![],
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}", self.path().join("/"))
    }
}

impl From<&Route> for seed::Url {
    fn from(route: &Route) -> Self {
        match route {
            Route::Home => seed::Url::new(),
            Route::Login => seed::Url::new().add_path_part("login"),
            Route::Register => seed::Url::new().add_path_part("register"),
            Route::Decks(username) => seed::Url::new()
                .add_path_part("user")
                .add_path_part(username.as_str())
                .add_path_part("decks"),
            Route::DeckCards(username, deck_id) => seed::Url::new()
                .add_path_part("user")
                .add_path_part(username.as_str())
                .add_path_part("decks")
                .add_path_part(format!("{}", deck_id))
                .add_path_part("cards"),
            Route::DeckSets(username, deck_id) => seed::Url::new()
                .add_path_part("user")
                .add_path_part(username.as_str())
                .add_path_part("decks")
                .add_path_part(format!("{}", deck_id))
                .add_path_part("sets"),
            Route::CardDetails(card_id) => seed::Url::new()
                .add_path_part("cards")
                .add_path_part(format!("{}", card_id)),
            Route::SetDetails(set_id) => seed::Url::new()
                .add_path_part("sets")
                .add_path_part(format!("{}", set_id)),
            Route::AddDeck(username) => seed::Url::new()
                .add_path_part("user")
                .add_path_part(username.as_str())
                .add_path_part("add-deck"),
            Route::DeckDetails(username, deck_id) => seed::Url::new()
                .add_path_part("user")
                .add_path_part(username.as_str())
                .add_path_part("decks")
                .add_path_part(format!("{}", deck_id)),
            Route::AddCard(username, deck_id) => seed::Url::new()
                .add_path_part("user")
                .add_path_part(username.as_str())
                .add_path_part("decks")
                .add_path_part(format!("{}", deck_id))
                .add_path_part("add-card"),
            Route::AddSet(username, deck_id) => seed::Url::new()
                .add_path_part("user")
                .add_path_part(username.as_str())
                .add_path_part("decks")
                .add_path_part(format!("{}", deck_id))
                .add_path_part("add-set"),
            Route::NotFound => seed::Url::new(),
        }
    }
}

impl TryFrom<Url> for Route {
    type Error = ();

    fn try_from(url: seed::Url) -> Result<Self, Self::Error> {
        let mut path = url.path().into_iter();
        match path.next().as_ref().map(|s| s.as_str()) {
            None | Some("") => Ok(Route::Home),
            Some("login") => Ok(Route::Login),
            Some("register") => Ok(Route::Register),
            Some("cards") => match path.next().as_ref().map(|s| s.as_str()) {
                Some(id) => id
                    .parse::<usize>()
                    .map(|card_id| Ok(Route::CardDetails(card_id)))
                    .unwrap_or(Err(())),
                _ => Err(()),
            },
            Some("sets") => match path.next().as_ref().map(|s| s.as_str()) {
                Some(id) => id
                    .parse::<usize>()
                    .map(|set_id| Ok(Route::SetDetails(set_id)))
                    .unwrap_or(Err(())),
                _ => Err(()),
            },
            Some("user") => match path.next().as_ref().map(|s| s.as_str()) {
                Some(username) => match path.next().as_ref().map(|s| s.as_str()) {
                    Some("add-deck") => Ok(Route::AddDeck(username.to_owned())),
                    Some("decks") => match path.next().as_ref().map(|s| s.as_str()) {
                        Some(deck_id) => deck_id
                            .parse::<usize>()
                            .map(|deck_id| match path.next().as_ref().map(|s| s.as_str()) {
                                Some("cards") => Ok(Route::DeckCards(username.to_owned(), deck_id)),
                                Some("sets") => Ok(Route::DeckSets(username.to_owned(), deck_id)),
                                Some("add-card") => {
                                    Ok(Route::AddCard(username.to_owned(), deck_id))
                                }
                                Some("add-set") => Ok(Route::AddSet(username.to_owned(), deck_id)),
                                None => Ok(Route::DeckDetails(username.to_owned(), deck_id)),
                                _ => Err(()),
                            })
                            .unwrap_or(Err(())),
                        None => Ok(Route::Decks(username.to_owned())),
                    },
                    _ => Err(()),
                },
                None => Err(()),
            },
            _ => Err(()),
        }
    }
}

pub struct RoutingModel {
    pub route: Route,
}

impl RoutingModel {
    pub fn new(url: Url) -> Self {
        Self {
            route: Route::try_from(url).unwrap_or(Route::NotFound),
        }
    }
}

pub fn update(action: &Msg, model: &mut RoutingModel) {
    match action {
        Msg::Routing(RoutingMessage::Navigate(r)) => {
            model.route = r.clone();
        }

        _ => {}
    }
}
