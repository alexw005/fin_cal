#![allow(non_snake_case)]
use dioxus::html::{EventData, label};
use dioxus::prelude::*;
use im_rc::HashMap;

// use crate::components::story_listing::StoryItem;

use log::{info};
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

fn App(cx: Scope) -> Element {
    let ammount =use_state(cx, String::new);
    fn calculator(e: HashMap<String,Vec<String>>) -> String{
        let data = e;
        info!("{:?}", data.get("loan_total"));
        let loan_total=data.get("loan_total").clone().unwrap()[0].parse::<f32>().unwrap();
        let interest_rate=data.get("interest_rate").clone().unwrap()[0].parse::<f32>().unwrap()/100.0;
        let loan_term=data.get("loan_term").clone().unwrap()[0].parse::<f32>().unwrap();
        let yearly_payments=12.0;
        // let extra_payment=data.get("extra_payment").clone().unwrap()[0].parse::<f32>().unwrap();

        let monthly_interest_rate:f32=interest_rate/yearly_payments;
        let total_payments_number:f32=loan_term*yearly_payments;
        let montly_payment:f32=(loan_total*monthly_interest_rate)/(1.0-(1.0+monthly_interest_rate).powf(-total_payments_number));
        info!("{:?}",montly_payment);
        
        montly_payment.to_string()
    }
    cx.render(rsx! {
        div{
            class:"h-screen bg-blue-300 text-center space-y-2 items-center flex justify-center flex-col",
            h1 {
                class:"text-lg text-black font-semibold",
                "Financial loan calculator"
            }
            form{
                class:"text-center space-y-2 sm:text-left flex flex-col items-center",
                onsubmit: move |event| 
                    ammount.set(calculator(event.data.values.clone().into())),        
                input{
                    r#type: "number",
                    min:"0",
                    class:"appearance-none border-2 border-purple-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-red-500",
                    placeholder:"Loan ammount, ie: 100000",
                    name: "loan_total",
                    required: true,
                },
                input{ 
                    r#type: "number",
                    min:"0",
                    step:"0.01",
                    class:"appearance-none border-2 border-purple-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-red-500",
                    name:"interest_rate",placeholder:"Interest Rate, ie: 5.5 for 5.5%",
                    required: true,
                },
                input{ 
                    r#type: "number",
                    min:"0",
                    class:"appearance-none border-2 border-purple-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-red-500",
                    name:"loan_term",placeholder:"Loan term in years, ie: 30",
                    required: true,
                },
                // label{
                //     "Extra payment"
                // },
                // input{ 
                //     r#type: "number",
                //     initial_value:"0",
                //     min:"0",
                //     class:"border-2 border-blue-500 bg-white h-10 px-5 pr-16 rounded-lg text-sm focus:outline-none",
                //     name:"extra_payment",placeholder:"extra_payment",
                // },
                input { class:"bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                r#type: "submit", },
            }
            h2{
                if ammount.trim().is_empty(){
                   ""
                }
                else{
                    "Assumsing number of yearly payments is 12, your monthly payment will be: "
                }
                "{ammount}"
            }
            
        }
    })
}

