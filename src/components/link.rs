use seed::{prelude::*, *};

use crate::Msg;

pub fn link(text: &str, href: &str) -> Node<Msg> {
    a![
        attrs! {
          At::Href => href,
          At::Class => "spectrum-Link"
        },
        text
    ]
}
