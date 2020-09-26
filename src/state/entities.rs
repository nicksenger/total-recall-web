use std::collections::HashMap;

use graphql_client::{GraphQLQuery, Response as GQLResponse};
use seed::prelude::Orders;

use crate::messages::{
    cards::{CardsMsg, GetCardsSuccessPayload},
    decks::{DecksMsg, GetDecksSuccessPayload},
    session::{ScoreValue, SessionMsg},
    sets::{GetSetsSuccessPayload, SetsMsg},
    ErrorPayload, Msg,
};
use crate::operations::{cards, decks, send_graphql_request, sets};

#[derive(Clone)]
pub struct Set {
    pub deck: usize,
    pub id: usize,
    pub name: String,
    pub owner: String,
    pub card_ids: Vec<usize>,
}

#[derive(Clone)]
pub struct Deck {
    pub created: u128,
    pub id: usize,
    pub language: String,
    pub name: String,
    pub owner: String,
}

#[derive(Clone)]
pub struct Card {
    pub id: usize,
    pub created: u128,
    pub last_seen: u128,
    pub front: String,
    pub back: String,
    pub score: Vec<ScoreValue>,
    pub audio: String,
    pub image: String,
    pub link: Option<String>,
}

#[derive(Clone)]
pub struct Language {
    pub id: usize,
    pub abbreviation: String,
    pub name: String,
}

pub struct EntitiesModel {
    pub cards: HashMap<usize, Card>,
    pub deck_cards: HashMap<usize, Vec<usize>>,
    pub deck_sets: HashMap<usize, Vec<usize>>,
    pub decks: HashMap<usize, Deck>,
    pub set_cards: HashMap<usize, Vec<usize>>,
    pub sets: HashMap<usize, Set>,
    pub languages: HashMap<usize, Language>,
}

impl EntitiesModel {
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
            deck_cards: HashMap::new(),
            deck_sets: HashMap::new(),
            decks: HashMap::new(),
            set_cards: HashMap::new(),
            sets: HashMap::new(),
            languages: HashMap::new(),
        }
    }
}

pub fn update(msg: &Msg, model: &mut EntitiesModel, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Cards(CardsMsg::GetCards(payload)) => {
            let deck_id = payload.deck_id;
            orders.perform_cmd(async move {
                Msg::Cards(CardsMsg::GetCardsFetched((
                    deck_id,
                    send_graphql_request(&cards::DeckCards::build_query(
                        cards::deck_cards::Variables {
                            deck_id: deck_id as i64,
                        },
                    ))
                    .await,
                )))
            });
        }

        Msg::Cards(CardsMsg::GetCardsFetched((
            deck_id,
            Ok(GQLResponse {
                data: Some(data),
                errors: None,
            }),
        ))) => {
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
                                    cards::deck_cards::ScoreValue::ZERO => ScoreValue::Zero,
                                    cards::deck_cards::ScoreValue::ONE => ScoreValue::One,
                                    cards::deck_cards::ScoreValue::TWO => ScoreValue::Two,
                                    cards::deck_cards::ScoreValue::THREE => ScoreValue::Three,
                                    cards::deck_cards::ScoreValue::FOUR => ScoreValue::Four,
                                    cards::deck_cards::ScoreValue::FIVE => ScoreValue::Five,
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

        Msg::Cards(CardsMsg::GetCardsFetched((
            _,
            Ok(GQLResponse {
                data: _,
                errors: Some(errors),
            }),
        ))) => {
            orders.send_msg(Msg::Cards(CardsMsg::GetCardsFailed(ErrorPayload {
                content: errors
                    .last()
                    .map(|e| e.message.to_owned())
                    .unwrap_or("Failed to retrieve cards.".to_owned()),
            })));
        }

        Msg::Cards(CardsMsg::GetCardsFetched(_)) => {
            orders.send_msg(Msg::Cards(CardsMsg::GetCardsFailed(ErrorPayload {
                content: "Failed to retrieve cards.".to_owned(),
            })));
        }

        Msg::Cards(CardsMsg::GetCardsSuccess(payload)) => {
            model.deck_cards.insert(
                payload.deck_id,
                payload.cards.iter().map(|c| c.id).collect(),
            );
            payload.cards.iter().cloned().for_each(|card| {
                model.cards.insert(card.id, card);
            });
        }

        Msg::Decks(DecksMsg::GetDecks(payload)) => {
            let username = payload.username.clone();
            orders.perform_cmd(async move {
                Msg::Decks(DecksMsg::GetDecksFetched((
                    username.clone(),
                    send_graphql_request(&decks::UserDecks::build_query(
                        decks::user_decks::Variables { username },
                    ))
                    .await,
                )))
            });
        }

        Msg::Decks(DecksMsg::GetDecksFetched((
            username,
            Ok(GQLResponse {
                data: Some(data),
                errors: None,
            }),
        ))) => {
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

        Msg::Decks(DecksMsg::GetDecksFetched((
            _,
            Ok(GQLResponse {
                data: _,
                errors: Some(errors),
            }),
        ))) => {
            orders.send_msg(Msg::Decks(DecksMsg::GetDecksFailed(ErrorPayload {
                content: errors
                    .last()
                    .map(|e| e.message.to_owned())
                    .unwrap_or("Failed to retrieve decks.".to_owned()),
            })));
        }

        Msg::Decks(DecksMsg::GetDecksFetched(_)) => {
            orders.send_msg(Msg::Cards(CardsMsg::GetCardsFailed(ErrorPayload {
                content: "Failed to retrieve decks.".to_owned(),
            })));
        }

        Msg::Decks(DecksMsg::GetDecksSuccess(payload)) => {
            payload.decks.iter().cloned().for_each(|deck| {
                model.decks.insert(deck.id, deck);
            });
        }

        Msg::Sets(SetsMsg::GetSets(payload)) => {
            let deck_id = payload.deck_id;
            orders.perform_cmd(async move {
                Msg::Sets(SetsMsg::GetSetsFetched((
                    deck_id,
                    send_graphql_request(&sets::UserSets::build_query(
                        sets::user_sets::Variables {
                            deck_id: deck_id as i64,
                        },
                    ))
                    .await,
                )))
            });
        }

        Msg::Sets(SetsMsg::GetSetsFetched((
            deck_id,
            Ok(GQLResponse {
                data: Some(data),
                errors: None,
            }),
        ))) => {
            orders.send_msg(Msg::Sets(SetsMsg::GetSetsSuccess(GetSetsSuccessPayload {
                deck_id: *deck_id,
                sets: data
                    .sets
                    .iter()
                    .map(|s| Set {
                        id: s.id as usize,
                        card_ids: s
                            .cards
                            .iter()
                            .map(|c| c.card_id.id as usize)
                            .collect::<Vec<usize>>(),
                        deck: *deck_id,
                        name: s.name.clone(),
                        owner: s.owner.username.clone(),
                    })
                    .collect(),
            })));
        }

        Msg::Sets(SetsMsg::GetSetsSuccess(payload)) => {
            model
                .deck_sets
                .insert(payload.deck_id, payload.sets.iter().map(|s| s.id).collect());
            payload.sets.iter().cloned().for_each(|set| {
                model.set_cards.insert(
                    set.id,
                    set.card_ids
                        .iter()
                        .filter(|card_id| model.cards.contains_key(card_id))
                        .cloned()
                        .collect(),
                );
                model.sets.insert(set.id, set);
            });
        }

        Msg::Decks(DecksMsg::DeleteDeckSuccess(payload)) => {
            model.decks.remove(&payload.deck_id);
        }

        Msg::Cards(CardsMsg::DeleteCardSuccess(payload)) => {
            model.cards.remove(&payload.card_id);
            model.deck_cards.iter_mut().for_each(|(_, card_ids)| {
                card_ids.retain(|id| *id != payload.card_id);
            });
            model.set_cards.iter_mut().for_each(|(_, card_ids)| {
                card_ids.retain(|id| *id != payload.card_id);
            });
        }

        Msg::Sets(SetsMsg::DeleteSetSuccess(payload)) => {
            model.sets.remove(&payload.set_id);
            model.deck_sets.iter_mut().for_each(|(_, set_ids)| {
                set_ids.retain(|id| *id != payload.set_id);
            });
        }

        Msg::Cards(CardsMsg::EditCardLinkSuccess(payload)) => {
            if let Some(card) = model.cards.get_mut(&payload.card_id) {
                card.link = Some(payload.link.clone());
            }
        }

        Msg::Session(SessionMsg::RateCardSuccess(payload)) => {
            if let Some(card) = model.cards.get_mut(&payload.card_id) {
                card.score.push(payload.rating.clone());
            }
        }

        Msg::Decks(DecksMsg::GetLanguagesSuccess(payload)) => {
            payload.languages.iter().cloned().for_each(|l| {
                model.languages.insert(l.id, l);
            });
        }

        Msg::Cards(_) => {}

        _ => {}
    }
}
