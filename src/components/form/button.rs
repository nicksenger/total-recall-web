use seed::{prelude::*, *};

use crate::Msg;

pub enum ButtonType {
    CTA,
    Primary,
    Secondary,
    Warning,
}

pub fn button<MsU: 'static>(
    text: &str,
    button_type: ButtonType,
    on_click: impl FnOnce(web_sys::Event) -> MsU + 'static + Clone,
    disabled: bool,
) -> Node<Msg> {
    let btn_class = format!("spectrum-Button spectrum-ButtonGroup-item {}", match button_type {
        ButtonType::CTA => "spectrum-Button--cta",
        ButtonType::Primary => "spectrum-Button--primary",
        ButtonType::Secondary => "spectrum-Button--secondary",
        ButtonType::Warning => "spectrum-Button--warning",
    });
    button![
        if disabled {
            attrs! { At::Class => btn_class, At::Disabled => "true" }
        } else {
            attrs! { At::Class => btn_class }
        },
        span![attrs! { At::Class => "spectrum-Button-label" }, text],
        ev(Ev::Click, |ev| {
            ev.prevent_default();
            on_click(ev)
        })
    ]
}
