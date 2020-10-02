use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::{pc, px, *};

use super::side_menu;
use crate::{messages::Msg, state::Model};

pub fn app_frame(model: &Model, content: Vec<Node<Msg>>) -> Node<Msg> {
    let menu_open = seed_hooks::use_state(|| false);

    div![
        s().background_color("var(--spectrum-global-color-gray-50)"),
        s().min_height("100vh"),
        div![
            s().left("0"),
            s().right("0"),
            s().background("rgba(0, 0, 0, 0.2)"),
            s().opacity("0"),
            s().pointer_events("none"),
            s().position("fixed"),
            s().top("0"),
            s().bottom("0"),
            s().z_index("0"),
            s().overflow("hidden"),
            s().media("@media screen and (max-width: 960px)").opacity(if menu_open.get() { "1" } else { "visible" }),
            s().media("@media screen and (max-width: 960px)").pointer_events(if menu_open.get() { "auto" } else { "none" }),
            s().media("@media screen and (max-width: 960px)").visibility("visible"),
            s().media("@media screen and (max-width: 960px)").display("block"),
            s().media("@media screen and (max-width: 960px)").transition(if menu_open.get() {
                "opacity var(--spectrum-global-animation-duration-200, 160ms) ease-in 0ms"
            } else {
                "opacity var(--spectrum-global-animation-duration-200, 160ms) ease-out 0ms, visibility 0ms linear calc(0ms + var(--spectrum-global-animation-duration-200, 160ms))"
            }),
            ev(Ev::Click, move |_| menu_open.set(false))
        ],
        div![
            s().border_bottom("1px solid var(--spectrum-global-color-gray-300)"),
            s().background_color("var(--spectrum-global-color-gray-50)"),
            s().display("none"),
            s().box_sizing("border-box"),
            s().height("var(--spectrum-global-dimension-size-600)"),
            s().padding("var(--spectrum-global-dimension-size-100)"),
            s().media("@media screen and (max-width: 960px)").display("block"),
            button![
                attrs! {
                    At::Class => "spectrum-ActionButton spectrum-ActionButton--quiet"
                },
                ev(Ev::Click, move |_| menu_open.set(!menu_open.get())),
            ]
        ],
        div![
            s().display("flex"),
            s().flex_direction("row"),
            s().min_height("100vh"),
            s().max_height(pc(100)),
            s().height(pc(100)),
            side_menu(model, menu_open.get()),
            div![
                s().flex_grow("1"),
                s().border_top("none"),
                s().border_bottom("none"),
                s().overflow_y("auto"),
                div![
                    s().background("var(--spectrum-global-color-gray-50)"),
                    s().display("flex"),
                    s().min_height(pc(100)),
                    s().flex_direction("column"),
                    s().justify_content("space-between"),
                    article![
                        s().max_width(px(1080)),
                        s().margin("var(--spectrum-global-dimension-size-500) auto"),
                        s().padding("0 var(--spectrum-global-dimension-size-700)"),
                        s().width("-webkit-fill-available"),
                        s().media("@media screen and (max-width: 960px)").padding("0 var(--spectrum-global-dimension-size-250)"),
                        s().media("@media screen and (max-width: 960px)").margin("var(--spectrum-global-dimension-size-100) auto"),
                        content
                    ],
                    div![
                        s().max_width(px(1080)),
                        s().margin("var(--spectrum-global-dimension-size-500) auto"),
                        s().padding("0 var(--spectrum-global-dimension-size-700)"),
                        s().width("-webkit-fill-available"),
                        hr![attrs! { At::Class => "spectrum-Divider spectrum-Divider--small" }],
                        footer![
                            s().background_color("var(--spectrum-global-color-gray-50)"),
                            div![
                                s().width(pc(100)),
                                s().max_width(px(1440)),
                                s().margin("0 auto"),
                                div![
                                    s().padding(px(20)),
                                    s().display("flex"),
                                    s().align_items("center"),
                                    s().justify_content("space-between"),
                                    s().flex_wrap("wrap"),
                                    a![s().visibility("hidden")],
                                    ul![
                                        s().list_style("none"),
                                        s().padding("5px 0"),
                                        s().margin("0"),
                                        s().display("flex"),
                                        s().flex_wrap("wrap"),
                                        s().font_size(px(11)),
                                        s().color("#4B4B4B"),
                                        li![
                                            s().color("var(--spectrum-body-text-color, var(--spectrum-alias-text-color))"),
                                            "Copyright © 2020 Nick Senger. All rights reserved."
                                        ]
                                    ]
                                ]
                            ]
                        ]
                    ]
                ]
            ]
        ]
    ]
}
