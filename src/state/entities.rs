use std::collections::HashMap;

use seed::prelude::Orders;

use crate::actions::{
    cards::CardsAction,
    decks::DecksAction,
    session::{ScoreValue, SessionAction},
    sets::SetsAction,
    GlobalAction,
};

#[derive(Clone)]
pub struct Set {
    pub deck: usize,
    pub id: usize,
    pub name: String,
    pub owner: String,
    pub card_ids: String,
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
    pub score: String,
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
    cards: HashMap<usize, Card>,
    deck_cards: HashMap<usize, Vec<usize>>,
    deck_sets: HashMap<usize, Vec<usize>>,
    decks: HashMap<usize, Deck>,
    set_cards: HashMap<usize, Vec<usize>>,
    sets: HashMap<usize, Set>,
    languages: HashMap<usize, Language>,
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

pub fn update(
    action: &GlobalAction,
    model: &mut EntitiesModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Cards(CardsAction::GetCardsSuccess(payload)) => {
            model.deck_cards.insert(
                payload.deck_id,
                payload.cards.iter().map(|c| c.id).collect(),
            );
            payload.cards.iter().cloned().for_each(|card| {
                model.cards.insert(card.id, card);
            });
        }

        GlobalAction::Decks(DecksAction::GetDecksSuccess(payload)) => {
            payload.decks.iter().cloned().for_each(|deck| {
                model.decks.insert(deck.id, deck);
            });
        }

        GlobalAction::Sets(SetsAction::GetSetsSuccess(payload)) => {
            model
                .deck_sets
                .insert(payload.deck_id, payload.sets.iter().map(|s| s.id).collect());
            payload.sets.iter().cloned().for_each(|set| {
                model.set_cards.insert(
                    set.id,
                    set.card_ids
                        .split(",")
                        .map(|s| s.parse::<usize>().unwrap())
                        .filter(|card_id| model.cards.contains_key(card_id))
                        .collect(),
                );
                model.sets.insert(set.id, set);
            });
        }

        GlobalAction::Decks(DecksAction::DeleteDeckSuccess(payload)) => {
            model.decks.remove(&payload.deck_id);
        }

        GlobalAction::Cards(CardsAction::DeleteCardSuccess(payload)) => {
            model.cards.remove(&payload.card_id);
            model.deck_cards.iter_mut().for_each(|(_, card_ids)| {
                card_ids.retain(|id| *id != payload.card_id);
            });
            model.set_cards.iter_mut().for_each(|(_, card_ids)| {
                card_ids.retain(|id| *id != payload.card_id);
            });
        }

        GlobalAction::Sets(SetsAction::DeleteSetSuccess(payload)) => {
            model.sets.remove(&payload.set_id);
            model.deck_sets.iter_mut().for_each(|(_, set_ids)| {
                set_ids.retain(|id| *id != payload.set_id);
            });
        }

        GlobalAction::Cards(CardsAction::EditCardLinkSuccess(payload)) => {
            if let Some(card) = model.cards.get_mut(&payload.card_id) {
                card.link = Some(payload.link.clone());
            }
        }

        GlobalAction::Session(SessionAction::RateCardSuccess(payload)) => {
            if let Some(card) = model.cards.get_mut(&payload.card_id) {
                let mut new_score = card.score.split(",").collect::<Vec<&str>>();
                new_score.push(match payload.rating {
                    ScoreValue::Zero => "0",
                    ScoreValue::One => "1",
                    ScoreValue::Two => "2",
                    ScoreValue::Three => "3",
                    ScoreValue::Four => "4",
                    ScoreValue::Five => "5",
                });
                card.score = new_score.join(",");
            }
        }

        GlobalAction::Decks(DecksAction::GetLanguagesSuccess(payload)) => {
            payload.languages.iter().cloned().for_each(|l| {
                model.languages.insert(l.id, l);
            });
        }

        _ => {}
    }
}
