use seed::{prelude::*, *};
use seed_style::{pc, px, *};

use crate::{
    messages::{authentication::AuthMsg, Msg},
    state::Model,
    Route, BASE_URI,
};

pub fn side_menu(model: &Model, menu_open: bool) -> Node<Msg> {
    div![
        s().min_height("100vh"),
        s().background_color("var(--spectrum-global-color-gray-75)"),
        s().display("flex"),
        s().flex_direction("column"),
        s().flex_grow("0"),
        s().flex_shrink("0"),
        s().max_width(pc(100)),
        s().transition("none"),

        s().media("@media screen and (max-width: 960px)").right("100%"),
        s().media("@media screen and (max-width: 960px)").position("fixed"),
        s().media("@media screen and (max-width: 960px)").top("0"),
        s().media("@media screen and (max-width: 960px)").bottom("0"),
        s().media("@media screen and (max-width: 960px)").z_index("100"),
        s().media("@media screen and (max-width: 960px)").transform(if menu_open { "translateX(100%)" } else { "translateX(0)" }),
        s().media("@media screen and (max-width: 960px)").transition("transform var(--spectrum-global-animation-duration-200, 160ms) ease-in-out"),
        a![
            attrs! { At::Href => Route::Home },
            s().flex_grow("0"),
            s().flex_shrink("0"),
            s().display("flex"),
            s().flex_direction("row"),
            s().align_items("center"),
            s().padding("var(--spectrum-global-dimension-size-350) var(--spectrum-global-dimension-size-300)"),
            s().text_decoration("none"),
            img![attrs! { At::Src => format!("{}/icon.png", BASE_URI) }, s().height(px(32)), s().margin_right("var(--spectrum-global-dimension-size-200)")],
            h2![
              s().color("var(--spectrum-heading-m-text-color, var(--spectrum-alias-heading-text-color))"),
              s().font_family("var(--spectrum-heading-m-text-font-family, var(--spectrum-alias-body-text-font-family))"),
              s().font_weight("var(--spectrum-heading-m-text-font-weight, var(--spectrum-alias-heading-text-font-weight-regular))"),
              s().font_size("var(--spectrum-heading-m-text-size, var(--spectrum-alias-heading-m-text-size))"),
              s().line_height("var(--spectrum-heading-m-text-line-height, var(--spectrum-alias-heading-text-line-height))"),
              s().font_style("var(--spectrum-heading-m-text-font-style, var(--spectrum-global-font-style-regular))"),
              s().letter_spacing("var(--spectrum-heading-m-text-letter-spacing, var(--spectrum-global-font-letter-spacing-none))"),
              s().text_transform("var(--spectrum-heading-m-text-transform, none)"),
              s().margin_top("0"),
              s().margin_bottom("0"),
              s().white_space("nowrap"),
              "Total Recall"
            ]
        ],
        div![
          s().padding("0 var(--spectrum-global-dimension-size-300) var(--spectrum-global-dimension-size-300) var(--spectrum-global-dimension-size-300)"),
          hr![attrs! { At::Class => "spectrum-Divider spectrum-Divider--small" }],
        ],
        div![
          s().overflow_x("hidden"),
          s().overflow_y("auto"),
          s().flex_grow("1"),
          s().padding("0px var(--spectrum-global-dimension-static-size-300, 24px) var(--spectrum-global-dimension-static-size-500, 40px)"),
          nav![
            if let Some(username) = &model.authentication.username {
              ul![
                attrs! { At::Class => "spectrum-SideNav spectrum-SideNav--multiLevel" },
                li![
                  attrs! {
                    At::Class => format!(
                      "spectrum-SideNav-item{}",
                      match model.routing.route {
                        Route::Decks(_) | Route::AddCard(_, _) | Route::AddSet(_, _) | Route::AddDeck(_) | Route::CardDetails(_) | Route::SetDetails(_) | Route::DeckSets(_, _) | Route::DeckCards(_, _) => " is-selected",
                        _ => ""
                      }
                    )
                  },
                  a![ attrs! { At::Class => "spectrum-SideNav-itemLink", At::Href => Route::Decks(username.clone()) }, "Decks"]
                ],
                li![
                  attrs! {
                    At::Class => format!(
                      "spectrum-SideNav-item{}",
                      match model.routing.route {
                        Route::Study => " is-selected",
                        _ => ""
                      }
                    )
                  },
                  a![ attrs! { At::Class => "spectrum-SideNav-itemLink", At::Href => Route::Study }, "Study"]
                ],
                li![
                  attrs! {
                    At::Class => format!(
                      "spectrum-SideNav-item{}",
                      match model.routing.route {
                        Route::Manual => " is-selected",
                        _ => ""
                      }
                    )
                  },
                  a![ attrs! { At::Class => "spectrum-SideNav-itemLink", At::Href => Route::Manual }, "User Manual"]
                ],
                li![
                  attrs! {
                    At::Href => Route::Home
                  },
                  a![
                    attrs! {
                      At::Class => "spectrum-SideNav-itemLink",
                      At::Href => Route::Home },
                      format!("Logout ({})", username),
                      ev(Ev::Click, |_| Msg::Authentication(AuthMsg::Logout)),
                  ]
                ]
              ]
            } else {
              ul![
                attrs! { At::Class => "spectrum-SideNav spectrum-SideNav--multiLevel" },
                li![
                  attrs! {
                    At::Class => format!(
                      "spectrum-SideNav-item{}",
                      match model.routing.route {
                        Route::Login => " is-selected",
                        _ => ""
                      }
                    )
                  },
                  a![ attrs! { At::Class => "spectrum-SideNav-itemLink", At::Href => Route::Login }, "Login"]
                ],
                li![
                  attrs! {
                    At::Class => format!(
                      "spectrum-SideNav-item{}",
                      match model.routing.route {
                        Route::Register => " is-selected",
                        _ => ""
                      }
                    )
                  },
                  a![ attrs! { At::Class => "spectrum-SideNav-itemLink", At::Href => Route::Register }, "Register"]
                ],
                li![
                  attrs! {
                    At::Class => format!(
                      "spectrum-SideNav-item{}",
                      match model.routing.route {
                        Route::Manual => " is-selected",
                        _ => ""
                      }
                    )
                  },
                  a![ attrs! { At::Class => "spectrum-SideNav-itemLink", At::Href => Route::Manual }, "User Manual"]
                ]
              ]
            }
          ]
        ],
    ]
}
