use crate::components::navbar::Navbar;
use crate::components::terminal_list::{TerminalList, TerminalItem, Command};
use dioxus::prelude::*;
use macros::md;

#[derive(Clone)]
struct Project {
    name: String,
    description: String,
}

#[component]
pub fn Projects() -> Element {
    let items = vec![
        TerminalItem {
            name: "my_file.md".to_string(),
            description: "A sample project description".to_string(),
        },
        TerminalItem {
            name: "my_file2.md".to_string(),
            description: "A sample project description".to_string(),
        },
        // Add more projects here as needed
    ];

    let command = Command {
        command: "ls".to_string(),
        flags: "-1F".to_string(),
        directory: "projects/".to_string(),
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
