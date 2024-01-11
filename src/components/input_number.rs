use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct InputProps {
    pub number: u32,
}

#[component]
pub fn InputNumber(cx:Scope,input:InputProps) -> Element {
    let number= input.number;

    cx.render(rsx!{
        input {
            padding: "0.5rem",
            position: "relative",
            "{number}"
        }
        div{
            "{number}"
        }
    })
}