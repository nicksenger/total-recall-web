use seed::{prelude::*, *};
use seed_style::{pc, px, *};

use crate::{messages::session::ScoreValue, Msg};

pub fn score_meter(score: &ScoreValue) -> Node<Msg> {
    let color = match score {
        ScoreValue::Zero | ScoreValue::One => "is-critical",
        ScoreValue::Two | ScoreValue::Three => "is-warning",
        ScoreValue::Four | ScoreValue::Five => "is-positive",
    };
    let width = match score {
        ScoreValue::Zero => pc(0),
        ScoreValue::One => pc(20),
        ScoreValue::Two => pc(40),
        ScoreValue::Three => pc(60),
        ScoreValue::Four => pc(80),
        ScoreValue::Five => pc(100),
    };

    div![
        s().media("@media screen and (max-width: 768px)").width(px(40)),
        C!["spectrum-ProgressBar", color],
        div![
            C!["spectrum-ProgressBar-track"],
            div![s().width(width), C!["spectrum-ProgressBar-fill"]]
        ]
    ]
}
