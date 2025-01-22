mod components;
mod views;
use dioxus::prelude::*;
use document::{Link, Script, Stylesheet};
use views::home::Home;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const FONT_CSS: Asset = asset!("/assets/font.css");
const MAIN_JS: Asset = asset!("/assets/js/main.js");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Link { rel: "icon", href: FAVICON }
        Stylesheet { href: MAIN_CSS }
        Stylesheet { href: FONT_CSS }
        body {
            Home {}
        }
        Script { src: MAIN_JS }
    }
}
