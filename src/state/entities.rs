use std::collections::HashMap;

use seed::{browser::web_storage::WebStorageError, prelude::*};
use serde::{Deserialize, Serialize};

use crate::messages::{
    cache::CacheMsg,
    cards::CardsMsg,
    decks::DecksMsg,
    session::{ScoreValue, SessionMsg},
    sets::SetsMsg,
    Msg,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Set {
    pub deck: usize,
    pub id: usize,
    pub name: String,
    pub owner: String,
    pub card_ids: Vec<usize>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Deck {
    pub created: u128,
    pub id: usize,
    pub language: String,
    pub name: String,
    pub owner: String,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Debug)]
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

#[derive(Clone, PartialEq, Eq, Debug)]
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

        Msg::Cache(CacheMsg::Hydrate) => {
            let cards: Result<HashMap<usize, Card>, WebStorageError> = LocalStorage::get("cards");
            if let Ok(cards) = cards {
                model.cards = cards;
            }
        }

        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::{cards::*, decks::*, session::*, sets::*};

    fn get_test_card(id: usize) -> Card {
        Card {
            id,
            audio: "http://foo.com/bar.mp3".to_owned(),
            image: "http://foo.com/bar.jpg".to_owned(),
            back: "hello".to_owned(),
            front: "hola".to_owned(),
            created: 12345,
            last_seen: 12345,
            link: None,
            score: vec![],
        }
    }

    fn get_test_deck(id: usize) -> Deck {
        Deck {
            id,
            name: "foo".to_owned(),
            owner: "waldo".to_owned(),
            created: 12345,
            language: "Zulu".to_owned(),
        }
    }

    fn get_test_set(id: usize, deck_id: usize, card_ids: Vec<usize>) -> Set {
        Set {
            id,
            card_ids,
            deck: deck_id,
            name: "foo".to_owned(),
            owner: "waldo".to_owned(),
        }
    }

    fn get_test_language(id: usize) -> Language {
        Language {
            id,
            name: "Zulu".to_owned(),
            abbreviation: "zl".to_owned(),
        }
    }

    #[test]
    fn get_cards_success() {
        let mut model = EntitiesModel::new();
        update(
            &Msg::Cards(CardsMsg::GetCardsSuccess(GetCardsSuccessPayload {
                cards: vec![get_test_card(123), get_test_card(456)],
                deck_id: 789,
            })),
            &mut model,
        );

        assert_eq!(model.cards.get(&123), Some(&get_test_card(123)));
        assert_eq!(model.cards.get(&456), Some(&get_test_card(456)));
        assert_eq!(model.deck_cards.get(&789), Some(&vec![123, 456]));
    }

    #[test]
    fn get_decks_success() {
        let mut model = EntitiesModel::new();
        update(
            &Msg::Decks(DecksMsg::GetDecksSuccess(GetDecksSuccessPayload {
                decks: vec![get_test_deck(123), get_test_deck(456)],
            })),
            &mut model,
        );

        assert_eq!(model.decks.get(&123), Some(&get_test_deck(123)));
        assert_eq!(model.decks.get(&456), Some(&get_test_deck(456)));
    }

    #[test]
    fn get_sets_success() {
        let mut model = EntitiesModel::new();
        model.cards.insert(123, get_test_card(123));
        model.cards.insert(456, get_test_card(456));
        update(
            &Msg::Sets(SetsMsg::GetSetsSuccess(GetSetsSuccessPayload {
                deck_id: 789,
                sets: vec![
                    get_test_set(1, 789, vec![123, 456]),
                    get_test_set(2, 789, vec![123, 456]),
                ],
            })),
            &mut model,
        );

        assert_eq!(
            model.sets.get(&1),
            Some(&get_test_set(1, 789, vec![123, 456]))
        );
        assert_eq!(
            model.sets.get(&2),
            Some(&get_test_set(2, 789, vec![123, 456]))
        );
        assert_eq!(model.set_cards.get(&1), Some(&vec![123, 456]));
        assert_eq!(model.set_cards.get(&2), Some(&vec![123, 456]));
        assert_eq!(model.deck_sets.get(&789), Some(&vec![1, 2]));
    }

    #[test]
    fn delete_deck_success() {
        let mut model = EntitiesModel::new();
        model.decks.insert(789, get_test_deck(789));
        update(
            &Msg::Decks(DecksMsg::DeleteDeckSuccess(DeleteDeckSuccessPayload {
                deck_id: 789,
            })),
            &mut model,
        );

        assert_eq!(model.decks.get(&789), None);
    }

    #[test]
    fn delete_card_success() {
        let mut model = EntitiesModel::new();
        model.cards.insert(123, get_test_card(123));
        model.deck_cards.insert(789, vec![123]);
        model.set_cards.insert(1, vec![123]);
        update(
            &Msg::Cards(CardsMsg::DeleteCardSuccess(DeleteCardSuccessPayload {
                card_id: 123,
            })),
            &mut model,
        );

        assert_eq!(model.cards.get(&123), None);
        assert_eq!(model.deck_cards.get(&789), Some(&vec![]));
        assert_eq!(model.set_cards.get(&1), Some(&vec![]));
    }

    #[test]
    fn delete_set_success() {
        let mut model = EntitiesModel::new();
        model.sets.insert(1, get_test_set(1, 789, vec![123, 456]));
        model.deck_sets.insert(789, vec![1]);
        update(
            &Msg::Sets(SetsMsg::DeleteSetSuccess(DeleteSetSuccessPayload {
                set_id: 1,
            })),
            &mut model,
        );

        assert_eq!(model.sets.get(&1), None);
        assert_eq!(model.deck_sets.get(&789), Some(&vec![]));
    }

    #[test]
    fn edit_card_link_success() {
        let mut model = EntitiesModel::new();
        model.cards.insert(123, get_test_card(123));
        update(
            &Msg::Cards(CardsMsg::EditCardLinkSuccess(EditCardLinkSuccessPayload {
                card_id: 123,
                link: "foo.com/bar".to_owned(),
            })),
            &mut model,
        );

        assert_eq!(
            model.cards.get(&123).map(|c| c.link.clone()).unwrap(),
            Some("foo.com/bar".to_owned())
        );
    }

    #[test]
    fn rate_card_success() {
        let mut model = EntitiesModel::new();
        model.cards.insert(123, get_test_card(123));
        update(
            &Msg::Session(SessionMsg::RateCardSuccess(RateCardSuccessPayload {
                card_id: 123,
                rating: ScoreValue::Two,
            })),
            &mut model,
        );

        assert_eq!(
            model.cards.get(&123).map(|c| c.score.last().unwrap()),
            Some(&ScoreValue::Two)
        );
    }

    #[test]
    fn get_languages_success() {
        let mut model = EntitiesModel::new();
        update(
            &Msg::Decks(DecksMsg::GetLanguagesSuccess(GetLanguagesSuccessPayload {
                languages: vec![get_test_language(8), get_test_language(12)],
            })),
            &mut model,
        );

        assert_eq!(model.languages.get(&8), Some(&get_test_language(8)));
        assert_eq!(model.languages.get(&12), Some(&get_test_language(12)));
    }
}
