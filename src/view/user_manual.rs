use seed::{prelude::*, *};

use crate::{
    messages::{authentication::AuthMsg, Msg},
    state::{routing::Route, Model},
};

pub fn view(model: &Model) -> Node<Msg> {
    p!["Coming soon.."]
}
