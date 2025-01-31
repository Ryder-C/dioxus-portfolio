use proc_macro2::TokenStream;
use quote::quote;
use std::{collections::HashMap, fs, path};

pub struct MarkdownBuilder {
    files: HashMap<String, Vec<MarkdownFile>>,
}
pub struct MarkdownFile {
    name: String,
    content: String,
}
pub struct MdDioxusArtifact {
    name: String,
    content: TokenStream,
    markdown_content: String,
}

impl MarkdownBuilder {
    pub fn from_path(path: &path::Path) -> Self {
        let dirs = fs::read_dir(path).unwrap();
        let mut files = HashMap::new();
        for top_dir in dirs.map(|d| d.unwrap()) {
            if top_dir.file_type().unwrap().is_dir() {
                let category = top_dir.file_name().into_string().unwrap();
                let md_files = fs::read_dir(top_dir.path()).unwrap();
                for md_file in md_files.map(|f| f.unwrap()) {
                    if !md_file.file_type().unwrap().is_file()
                        || md_file.path().as_path().extension().unwrap() != "md"
                    {
                        panic!("Wrong file type in content directory");
                    }
                    let name = md_file.file_name().into_string().unwrap()
                        [..md_file.file_name().len() - 3]
                        .to_string();
                    let content = fs::read_to_string(md_file.path()).unwrap();
                    files
                        .entry(category.clone())
                        .or_insert(vec![])
                        .push(MarkdownFile { name, content });
                }
            }
        }
        Self { files }
    }

    pub fn synthesize(self) -> Vec<MdDioxusArtifact> {
        self.files
            .into_iter()
            .map(|(category, files)| {
                files
                    .into_iter()
                    .map(|file| {
                        let name = format!("__build_{category}_{}", file.name);
                        let path_name = format!("{name}.md");
                        let content = quote! {
                            ::dioxus_markdown::Markdown {
                                content: include_str!(#path_name),
                            }
                        };

                        MdDioxusArtifact {
                            name,
                            content,
                            markdown_content: file.content,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }
}

impl MdDioxusArtifact {
    pub fn emit(self, path: &path::Path) {
        // markdown
        fs::write(
            path.join(format!("{}.md", self.name)),
            self.markdown_content,
        )
        .unwrap();
        // rust
        fs::write(
            path.join(format!("{}.rs", self.name)),
            format!("rsx! {{ {} }}", self.content),
        )
        .unwrap();
    }
}
