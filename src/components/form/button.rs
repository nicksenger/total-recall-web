use seed::{prelude::*, *};

use crate::Msg;

pub enum ButtonType {
    CTA,
    Primary,
    Secondary,
    Warning,
    Action,
}

pub fn button<MsU: 'static>(
    text: &str,
    button_type: ButtonType,
    on_click: impl FnOnce(web_sys::Event) -> MsU + 'static + Clone,
    disabled: bool,
) -> Node<Msg> {
    let btn_class = match button_type {
        ButtonType::CTA => "spectrum-Button spectrum-ButtonGroup-item spectrum-Button--cta",
        ButtonType::Primary => "spectrum-Button spectrum-ButtonGroup-item spectrum-Button--primary",
        ButtonType::Secondary => "spectrum-Button spectrum-ButtonGroup-item spectrum-Button--secondary",
        ButtonType::Warning => "spectrum-Button spectrum-ButtonGroup-item spectrum-Button--warning",
        ButtonType::Action => "spectrum-ActionButton spectrum-ActionButton--quiet spectrum-ActionGroup-item"
    };
    let label_class = match button_type {
        ButtonType::Action => "spectrum-ActionButton-label",
        _ => "spectrum-Button-label"
    };
    button![
        C![btn_class],
        attrs! { At::Disabled => disabled.as_at_value() },
        span![C![label_class], text],
        ev(Ev::Click, |ev| {
            ev.prevent_default();
            on_click(ev)
        })
    ]
}
