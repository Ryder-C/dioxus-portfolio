use dioxus::prelude::*;
use macros::md;

#[component]
pub fn Blog() -> Element {
    md! { "blog", "blog0" }
}
