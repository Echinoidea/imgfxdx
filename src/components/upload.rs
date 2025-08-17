use dioxus::prelude::*;
use dioxus::{html::HasFileData, prelude::dioxus_elements::FileEngine};

#[derive(Clone, Debug)]
struct ImageData {
    name: String,
    size: u64,
    data_url: String,
}

#[component]
pub fn Upload() -> Element {
    let mut files_uploaded: Signal<Vec<String>> = use_signal(Vec::new);

    rsx! {
        input {
            r#type: "file",
            accept: ".png,.jpg,.jpeg",
            multiple: false,
            onchange: move |evt: FormEvent| {
                async move {
                    if let Some(file_engine) = evt.files() {
                        let files = file_engine.files();
                        for file_name in &files {
                            if let Some(file) = file_engine.read_file_to_string(file_name).await {
                                files_uploaded.write().push(file);
                            }
                        }
                    }
                }
            }
        }
    }
}
