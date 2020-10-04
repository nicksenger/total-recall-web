use seed::{prelude::*, *};
use seed_style::*;

use super::ButtonType;
use crate::Msg;

pub fn button_link(text: &str, button_type: ButtonType, href: &str) -> Node<Msg> {
    let btn_class = format!(
        "spectrum-Button spectrum-ButtonGroup-item {}",
        match button_type {
            ButtonType::CTA => "spectrum-Button--cta",
            ButtonType::Primary => "spectrum-Button--primary",
            ButtonType::Secondary => "spectrum-Button--secondary",
            ButtonType::Warning => "spectrum-Button--warning",
            ButtonType::Action =>
                "spectrum-ActionButton spectrum-ActionButton--quiet spectrum-ActionGroup-item",
        }
    );
    a![
        C![btn_class],
        attrs! { At::Href => href },
        span![C!["spectrum-Button-label"], text],
        s().text_decoration("none"),
    ]
}
