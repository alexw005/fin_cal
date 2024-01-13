#![allow(non_snake_case)]
use chrono::format::Numeric;
use dioxus::html::EventData;
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use im_rc::HashMap;

// use crate::components::story_listing::StoryItem;
use crate::components::input_number::InputProps;
use crate::components::fancy_button::FancyButton;

use log::{debug, warn,info};
fn main() {
    // launch the web app
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}
mod components{
    // pub mod story_listing;
    pub mod input_number;
    pub mod fancy_button;
}
// create a component that renders a div with the text "Hello, world!""

fn App(cx: Scope) -> Element {
    // let form_input = use_state(cx, HashMap::new);
    let ammount =use_state(cx, String::new);
    let draft = use_state(cx, String::new);
    fn calculator(e: HashMap<String,Vec<String>>) -> String{
        let data = e;
        info!("{:?}", data.get("loan_total"));
        let loan_total=data.get("loan_total").clone().unwrap()[0].parse::<u32>().unwrap();
        let age=data.get("age").clone().unwrap()[0].parse::<u32>().unwrap();
        info!("{:?}", loan_total*age);
        
        (loan_total*age).to_string()
    }
    cx.render(rsx! {
        div{
            class:"text-center space-y-2",
            h1 {
                class:"text-lg text-black font-semibold",
                "Financial loan calculator"
            }
            form{
                class:"text-center space-y-2 sm:text-left flex flex-col items-center",
                onsubmit: move |event| 
                    // info!("{:?}",event.data.values.clone()),
                        // form_input.set(event.data.values.clone().into()),
                        ammount.set(calculator(event.data.values.clone().into())),
                        
                input{
                    r#type: "number",
                    min:"0",
                    class:"border-2 border-blue-500 bg-white h-10 px-5 pr-16 rounded-lg text-sm focus:outline-none",
                    placeholder:"Loan ammount",
                    name: "loan_total",
                    oninput: move |evt| draft.set(evt.value.clone()),},
                input{ 
                    r#type: "number",
                    min:"0",
                    class:"border-2 border-blue-500 bg-white h-10 px-5 pr-16 rounded-lg text-sm focus:outline-none",
                    name:"age",placeholder:"age",},
                input { class:"bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                r#type: "submit", },
            }
            h1{
                "{ammount}"
            }
            // info!("{:?}",form_input.get().get("name"))
            
        }
    })
}

