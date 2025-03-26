use crate::components::navbar::Navbar;
use crate::components::terminal_list::{TerminalList, TerminalItem, Command};
use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    let items = vec![
        TerminalItem {
            name: "first_post.md".to_string(),
            description: "My first blog post".to_string(),
        },
        // Add more blog posts here as needed
    ];

    let command = Command {
        command: "ls".to_string(),
        flags: "-1F".to_string(),
        directory: "blog/".to_string(),
    };

    rsx! {
        div {
            class: "terminal-history",
            Navbar {}
            TerminalList {
                command: command,
                items: items,
            }
        }
    }
}
