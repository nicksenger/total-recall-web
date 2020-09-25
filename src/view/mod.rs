use seed::{prelude::*, *};

use crate::{actions::GlobalAction, state::Model};

pub fn root(_model: &Model) -> Node<GlobalAction> {
    h3!["Total Recall Web"]
}
