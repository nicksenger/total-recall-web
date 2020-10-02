use seed::{prelude::*, *};

use crate::{components::link, messages::Msg, state::Model};

pub fn view(_model: &Model) -> Node<Msg> {
    div![
        header![
            attrs! { At::Class => "spectrum-CSSComponent-heading" },
            p![
                h1![
                    attrs! { At::Class => "spectrum-Heading spectrum-Heading--XXXL spectrum-Heading-serif" },
                    "User Manual"
                ],
                br![],
                br![],
                h3![
                    attrs! { At::Class => "spectrum-Heading spectrum-Heading--M" },
                    "Getting started"
                ],
                br![],
                p![
                    attrs! { At::Class => "spectrum-Body--M" },
                    "First, make a deck by clicking on \"Create Deck\" from the Decks page. Choose a name and target",
                    " language for your deck, then press \"Go!\". Total Recall will download audio in the selected ",
                    " language for each card that you add to your deck which can be played back while studying.",
                    " To see a list of supported languages, ",
                    link("click here", "https://translate.google.com/about/languages/"), "."
                ],
                br![],
                br![],
                h3![
                    attrs! { At::Class => "spectrum-Heading spectrum-Heading--M" },
                    "Adding cards"
                ],
                br![],
                p![
                    attrs! { At::Class => "spectrum-Body--M" },
                    "To add a card to an existing deck, first select the desired deck from the Decks page to view the ",
                    "list of all cards for that deck. From the cards page, select \"Create Card,\" then enter ",
                    "the meaning of the card in your native language in the section labeled \"Front\", and the word in the ",
                    "target foreign language in the section labeled \"Back.\" Optionally, you may add a contextual link which ",
                    "will be displayed on the back of the card for quick navigation during study sessions."
                ],
                br![],
                br![],
                h3![
                    attrs! { At::Class => "spectrum-Heading spectrum-Heading--M" },
                    "Studying"
                ],
                br![],
                p![
                    attrs! { At::Class => "spectrum-Body--M" },
                    "From the \"Cards\" screen, select as many cards as you'd like to study, then click \"Study\" to begin a ",
                    "session. The cards will be shuffled randomly, and you will be shown the word on the front of the first card. ",
                    "Try to recall the foreign language word corresponding to the displayed word in your native language (ideally ",
                    "you should say it out loud), then ",
                    "press \"Flip.\" The audio for the foreign language word will be played, and the text and image for the word ",
                    "will be shown. Rate your ability to recall the word based on the following scale:",
                    ul![
                        li![strong!["0"], " - No recollection at all."],
                        li![strong!["1"], " - Incorrect response, but the correct one remembered."],
                        li![strong!["2"], " - Incorect response, but the correct one seemed easy to recall."],
                        li![strong!["3"], " - Correct response recalled with serious difficulty."],
                        li![strong!["4"], " - Correct response recalled with some hesitation."],
                        li![strong!["5"], " - Perfect response."],
                    ],
                    br![],
                    "Once each card in the deck has been reviewed, any cards which were scored 3 or lower will continue to be shown ",
                    "until a score over 3 is achieved. Only the first response for a card will be used to determine when it should be ",
                    "reviewed again."
                ],
                br![],
                br![],
                h3![
                    attrs! { At::Class => "spectrum-Heading spectrum-Heading--M" },
                    "Managing cards with sets"
                ],
                br![],
                p![
                    attrs! { At::Class => "spectrum-Body--M" },
                    "If you've added a large number of words to a Total Recall deck, you may quickly become overwhelmed by the number of ",
                    "cards due for review, and the sheer volume of unfamiliar material will have a detrimental impact on your ability to ",
                    "learn any new words at all. In these cases it's helpful to assign cards to particular sets which allow for quickly ",
                    "reviewing a group of words. To add a set, first select the cards you would like to group as if you were beginning a ",
                    "study session, then click on \"Create Set\". You will be prompted to enter a name for the set, and then forwarded to ",
                    "the set list page for the deck. To begin a session including all cards from one or more sets, simply select the sets ",
                    "that you would like to review, then press \"Study\" to begin the session."
                ]
            ]
        ],
    ]
}
