use seed::{prelude::*, *};

use crate::{messages::Msg, state::Model};

pub fn view(_model: &Model) -> Node<Msg> {
    div![h3!["404"], p!["The specified page does not exist."]]
}
