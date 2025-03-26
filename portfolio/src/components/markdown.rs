use dioxus::prelude::*;
use macros::md;

enum ContentType {
    Projects,
    Blog,
}

struct MarkdownProps {
    content_type: ContentType,
    file_name: String,
}

#[component]
pub fn Markdown(props: MarkdownProps) -> Element {
    let content_type = props.content_type;
    let file_name = props.file_name;

    match content_type {
        ContentType::Projects => md! { "projects", "my_file" },
        ContentType::Blog => md! { "blog", "blog0" },
    }
}