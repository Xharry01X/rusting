use dioxus::{html::g::class, prelude::*};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {

    let mut item: Signal<String> = use_signal(|| { String::new() });
    let mut items: Signal<Vec<String>> = use_signal(Vec::<String>::new);
    rsx! {
        div {
            div {
                class: "header",
                input {
                  type: "text",
                  class: "input",
                  oninput: move |event| {
                    item.set(event.value());
                  },
                  onkeydown: move |event| {
                    if event.code().to_string() == "Enter".to_string() {
                        println!("{:?}", item);
                        items.write().push(item());
                    }
                  }
                }
            },
            div {
                for i in 1..5 {
                    div { 
                        class: "item",
                        label { { i.to_string() } }

                        button { 
                            class: "delete-button",
                            "Delete"
                         }
                     }
                }
            }
        }
    }
}
