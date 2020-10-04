use seed::{prelude::*, *};

use crate::Msg;

pub fn link(text: &str, href: &str) -> Node<Msg> {
    a![
        C!["spectrum-Link"],
        attrs! {
          At::Href => href,
        },
        text
    ]
}
