use seed::{prelude::*, *};

use crate::{
    components::{button, ButtonType},
    Msg,
};

pub fn pager(
    current_page: usize,
    pages: usize,
    on_page: impl Fn(usize) + Clone + 'static,
) -> Node<Msg> {
    nav![
        C!["spectrum-Pagination", "spectrum-Pagination--listing"],
        if current_page as i32 - 1 > 1 {
            let handler = on_page.clone();
            vec![
                button("1", ButtonType::Action, move |_| handler(0), false),
                button("...", ButtonType::Action, |_| {}, true),
            ]
        } else {
            vec![]
        },
        (0..pages)
            .filter(|i| (current_page as i32 - *i as i32).abs() <= 1)
            .take(3)
            .map(|i| {
                let handler = on_page.clone();
                button(
                    format!("{}", i + 1).as_str(),
                    ButtonType::Action,
                    move |_| handler(i),
                    false,
                )
            }),
        if current_page + 1 < pages {
            let handler = on_page.clone();
            vec![
                button("...", ButtonType::Action, |_| {}, true),
                button(format!("{}", pages).as_str(), ButtonType::Action, move |_| handler(pages), false),
            ]
        } else {
            vec![]
        },
    ]
}
