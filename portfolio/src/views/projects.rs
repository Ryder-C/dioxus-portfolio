use dioxus::prelude::*;
use macros::md;

#[component]
pub fn Projects() -> Element {
    md! { "projects", "my_file" }
}
