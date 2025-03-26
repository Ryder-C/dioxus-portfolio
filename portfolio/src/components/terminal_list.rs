use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Command {
    pub command: String,
    pub flags: String,
    pub directory: String,
}

#[derive(Clone, Props, PartialEq)]
pub struct TerminalListProps {
    command: Command,
    items: Vec<TerminalItem>,
}

#[derive(Clone, PartialEq)]
pub struct TerminalItem {
    pub name: String,
    pub description: String,
}

#[component]
pub fn TerminalList(props: TerminalListProps) -> Element {
    let directory = props.command.directory.trim_end_matches('/');
    
    rsx! {
        div {
            class: "command-block",
            div {
                class: "ls-elem",
                p {
                    class: "host",
                    "ryder-solutions"
                }
                p {
                    class: "path",
                    "~"
                }
                p {
                    class: "term",
                    "$"
                }
                p {
                    class: "command",
                    "{props.command.command}"
                }
                p {
                    class: "flags",
                    "{props.command.flags}"
                }
                p {
                    class: "directory",
                    "{props.command.directory}"
                }
            }
            div {
                class: "animate",
                for item in props.items {
                    div {
                        class: "project-item",
                        Link {
                            class: "menu-item file",
                            to: "/{directory}/{item.name}",
                            div {
                                class: "project-title",
                                "{item.name}"
                            }
                            div {
                                class: "project-description",
                                "{item.description}"
                            }
                        }
                    }
                }
            }
        }
    }
} 