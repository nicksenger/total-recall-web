use seed::{prelude::*, *};

use crate::{messages::Msg, state::Model};

pub fn root(_model: &Model) -> Node<Msg> {
    h3!["Total Recall Web"]
}
