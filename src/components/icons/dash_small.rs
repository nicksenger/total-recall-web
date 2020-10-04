use seed::{prelude::*, *};

use crate::messages::Msg;

pub fn dash_small() -> Vec<Node<Msg>> {
    vec![
        path![
            C!["spectrum-UIIcon--large"],
            attrs! {
              At::D => "M10.99 5H1.01a1 1 0 000 2h9.98a1 1 0 100-2z"
            }
        ],
        path![
            C!["spectrum-UIIcon--medium"],
            attrs! {
              At::D => "M8 4H2a1 1 0 000 2h6a1 1 0 000-2z"
            }
        ],
    ]
}
