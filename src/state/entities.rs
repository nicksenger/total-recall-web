use std::collections::HashMap;

use crate::messages::{
    cards::CardsMsg,
    decks::DecksMsg,
    session::{ScoreValue, SessionMsg},
    sets::SetsMsg,
    Msg,
};

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

pub fn update(msg: &Msg, model: &mut EntitiesModel) {
    match msg {
        Msg::Cards(CardsMsg::GetCardsSuccess(payload)) => {
            model.deck_cards.insert(
                payload.deck_id,
                payload.cards.iter().map(|c| c.id).collect(),
            );
            payload.cards.iter().cloned().for_each(|card| {
                model.cards.insert(card.id, card);
            });
        }

        Msg::Decks(DecksMsg::GetDecksSuccess(payload)) => {
            payload.decks.iter().cloned().for_each(|deck| {
                model.decks.insert(deck.id, deck);
            });
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

        _ => {}
    }
}
