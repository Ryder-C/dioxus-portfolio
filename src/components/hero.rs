use dioxus::prelude::*;

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
                ListElement { typ: Directory, name: "projects" }
                ListElement { typ: Directory, name: "blog" }
                ListElement { typ: File, name: "about_me" }
                ListElement { typ: File, name: "skills" }
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
    name: String,
}

#[component]
fn ListElement(props: ListElementProps) -> Element {
    match props.typ {
        EntryType::Directory => rsx! {
            a {
                class: "menu-item directory",
                "{props.name}/"
            }
        },
        EntryType::File => rsx! {
            a {
                class: "menu-item file",
                "{props.name}.md"
            }
        },
    }
}
