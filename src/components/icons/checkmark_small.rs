use seed::{prelude::*, *};

use crate::messages::Msg;

pub fn checkmark_small() -> Vec<Node<Msg>> {
    vec![
        path![
            C!["spectrum-UIIcon--large"],
            attrs! {
              At::D => "M4.5 11a.999.999 0 01-.788-.385l-3-4a1 1 0 111.576-1.23L4.5 8.376l5.212-6.99a1 1 0 111.576 1.23l-6 8A.999.999 0 014.5 11z"
            }
        ],
        path![
            C!["spectrum-UIIcon--medium"],
            attrs! {
              At::D => "M3.788 9A.999.999 0 013 8.615l-2.288-3a1 1 0 111.576-1.23l1.5 1.991 3.924-4.991a1 1 0 111.576 1.23l-4.712 6A.999.999 0 013.788 9z"
            }
        ],
    ]
}
