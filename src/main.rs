#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use crate::components::story_listing::StoryItem;
use crate::components::input_number::InputProps;
fn main() {
    // launch the web app
    dioxus_web::launch(App);
}
mod components{
    pub mod story_listing;
    pub mod input_number;
}
// create a component that renders a div with the text "Hello, world!""

fn App(cx: Scope) -> Element {
    let mut number:u32 = 123;
    cx.render(rsx! {
        h1 {
            "Financial loan calculator"
        }
        components::story_listing:: StoryListing {
            story: StoryItem {
                id: 0,
                title: "hello hackernews".to_string(),
                url: None,
                text: None,
                by: "Author".to_string(),
                score: 0,
                descendants: 0,
                kids: vec![],
                r#type: "".to_string(),
            }
        }
        components::input_number:: InputNumber {
            input: InputProps {
                number: number,
            }
        }
    })
}

