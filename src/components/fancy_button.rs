use dioxus::prelude::*;

#[derive(Props)]
pub struct FancyButtonProps<'a> {
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn FancyButton<'a>(cx: Scope<'a, FancyButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx!(button {
        class: "fancy-button",
        onclick: move |evt| cx.props.on_click.call(evt),
        "click me pls."
    }))
}
