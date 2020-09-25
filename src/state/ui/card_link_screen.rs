use seed::prelude::Orders;

use crate::actions::{cards::CardsAction, GlobalAction};

pub struct CardLinkScreenModel {
    link: String,
}

impl CardLinkScreenModel {
    pub fn new() -> Self {
        Self {
            link: "".to_owned(),
        }
    }
}

pub fn update(
    action: &GlobalAction,
    model: &mut CardLinkScreenModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Cards(CardsAction::ViewCardLink(payload)) => {
            model.link = payload.link.clone()
        }

        _ => {}
    }
}
