use seed::prelude::Orders;

use crate::messages::{cards::CardsMsg, Msg};

pub struct CardLinkScreenModel {
    pub link: String,
}

impl CardLinkScreenModel {
    pub fn new() -> Self {
        Self {
            link: "".to_owned(),
        }
    }
}

pub fn update(
    action: &Msg,
    model: &mut CardLinkScreenModel,
    orders: &mut impl Orders<Msg>,
) {
    match action {
        Msg::Cards(CardsMsg::ViewCardLink(payload)) => {
            model.link = payload.link.clone()
        }

        _ => {}
    }
}
