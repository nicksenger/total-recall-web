use seed::{prelude::*, *};
use seed_style::{px, *};

use crate::Msg;

pub fn action_bar(children: Vec<Node<Msg>>) -> Node<Msg> {
    div![
        s().padding(px(0)),
        C!["spectrum-ActionBar", "is-open"],
        div![
            C!["spectrum-Popover spectrum-ActionBar-popover is-open"],
            children
        ]
    ]
}
