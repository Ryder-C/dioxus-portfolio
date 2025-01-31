use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Hero() -> Element {
    use EntryType::*;
    rsx! {
        div {
            class: "ls",
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
                    "ls"
                }
                p {
                    class: "flags",
                    "-F"
                }
                p {
                    class: "directory",
                    "."
                }
            }
            div {
                class: "animate ",
                ListElement { typ: Directory, name: "projects", route: Route::Projects {} }
                ListElement { typ: Directory, name: "blog", route: Route::Blog {} }
                ListElement { typ: File, name: "about_me", route: Route::Home {} }
                ListElement { typ: File, name: "skills", route: Route::Home {} }
            }
        }
    }
}

#[derive(PartialEq, Clone)]
enum EntryType {
    Directory,
    File,
}

#[derive(PartialEq, Clone, Props)]
struct ListElementProps {
    typ: EntryType,
    route: Route,
    name: String,
}

#[component]
fn ListElement(props: ListElementProps) -> Element {
    match props.typ {
        EntryType::Directory => rsx! {
            Link {
                class: "menu-item directory",
                to: props.route,
                "{props.name}/"
            }
        },
        EntryType::File => rsx! {
            Link {
                class: "menu-item file",
                to: props.route,
                "{props.name}.md"
            }
        },
    }
}
