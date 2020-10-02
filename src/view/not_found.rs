use seed::{prelude::*, *};

use crate::{messages::Msg, state::Model};

pub fn view(_model: &Model) -> Node<Msg> {
    div![
        header![
            attrs! { At::Class => "spectrum-CSSComponent-heading" },
            h1![
                attrs! { At::Class => "spectrum-Heading spectrum-Heading--XXXL spectrum-Heading-serif" },
                "404"
            ],
        ],
        p![
            "The specified page does not exist.",
            attrs! { At::Class => "spectrum-Body spectrum-Body--M" }
        ],
    ]
}
