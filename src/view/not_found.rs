use seed::{prelude::*, *};

use crate::{messages::Msg, state::Model};

pub fn view(_model: &Model) -> Node<Msg> {
    div![
        header![
            C!["spectrum-CSSComponent-heading"],
            h1![
                C!["spectrum-Heading spectrum-Heading--XXXL spectrum-Heading-serif"],
                "404"
            ],
        ],
        p![
            C!["spectrum-Body spectrum-Body--M"],
            "The specified page does not exist.",
        ],
    ]
}
