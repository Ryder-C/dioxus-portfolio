use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            class: "navbar",
            div {
                class: "navbar-content",
                div {
                    class: "navbar-brand",
                    Link {
                        class: "host",
                        to: Route::Home {},
                        "ryder-solutions"
                    }
                    p {
                        class: "term",
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
                }
                div {
                    class: "navbar-links",
                    Link {
                        class: "nav-link directory",
                        to: Route::Projects {},
                        "projects/"
                    }
                    Link {
                        class: "nav-link directory",
                        to: Route::Blog {},
                        "blog/"
                    }
                    Link {
                        class: "nav-link file",
                        to: Route::Home {},
                        "about_me.md"
                    }
                    Link {
                        class: "nav-link file",
                        to: Route::Home {},
                        "skills.md"
                    }
                    Link {
                        class: "nav-link file",
                        to: Route::Home {},
                        "resume.pdf"
                    }
                }
            }
        }
    }
}
