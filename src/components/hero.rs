use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "ls",
            div {
                class: "ls-elem",
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
                    "ryder-solutions"
                }
            }
            div {
                class: "animate",
                ListElement { is_dir: true, name: "projects" }
                ListElement { is_dir: true, name: "blog" }
                ListElement { is_dir: false, name: "about_me" }
                ListElement { is_dir: false, name: "skills" }
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
struct ListElementProps {
    is_dir: bool,
    name: String,
}

#[component]
fn ListElement(props: ListElementProps) -> Element {
    rsx! {
        a {
            class: if props.is_dir { "menu-item directory" } else { "menu-item file" },
            if props.is_dir { "{props.name}/" } else { "{props.name}.md" }
        }
    }
}
