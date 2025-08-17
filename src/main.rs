use backend::{and, or, xor};
use clap::builder::styling::RgbColor;
use components::{ColorPicker, EffectItem, NavBar};
use dioxus::prelude::*;
use image::{DynamicImage, RgbaImage};
use routes::Home;
use utils::{create_preview_data_url, is_image_file, rgba_image_to_data_url};

mod backend;
mod components;
mod routes;
mod utils;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut uploaded_image = use_signal(|| None::<DynamicImage>);
    let mut original_image_url = use_signal(|| None::<String>);
    let mut processed_image_url = use_signal(|| None::<String>);
    let mut upload_status = use_signal(|| String::new());
    let mut is_processing = use_signal(|| false);
    let mut selected_effect = use_signal(|| "or".to_string());
    let color = use_signal(|| RgbColor(0, 0, 0));

    // Handle file upload
    let handle_file_upload = move |evt: Event<FormData>| {
        if let Some(file_engine) = evt.files() {
            let files = file_engine.files();
            if let Some(file_name) = files.get(0) {
                let file_name = file_name.clone(); // Clone the file name to move into async block

                // Check if it's an image file first
                if is_image_file(&file_name) {
                    upload_status.set("Loading image...".to_string());

                    // Spawn async task to handle file reading
                    spawn(async move {
                        if let Some(file_data) = file_engine.read_file(&file_name).await {
                            // Load the image from bytes
                            match image::load_from_memory(&file_data) {
                                Ok(dynamic_image) => {
                                    // Store the original image
                                    uploaded_image.set(Some(dynamic_image.clone()));

                                    // Create preview of original image
                                    match create_preview_data_url(&dynamic_image) {
                                        Ok(data_url) => {
                                            original_image_url.set(Some(data_url));
                                            upload_status.set(format!(
                                                "Image '{}' loaded successfully!",
                                                file_name
                                            ));
                                            processed_image_url.set(None); // Clear any previous processed image
                                        }
                                        Err(e) => {
                                            upload_status
                                                .set(format!("Error creating preview: {}", e));
                                        }
                                    }
                                }
                                Err(e) => {
                                    upload_status.set(format!("Error loading image: {}", e));
                                }
                            }
                        } else {
                            upload_status.set("Failed to read file data".to_string());
                        }
                    });
                } else {
                    upload_status.set(
                        "Please select a valid image file (JPG, PNG, GIF, WebP, BMP)".to_string(),
                    );
                }
            }
        }
    };

    // Handle processing with OR function
    let apply_or_function = move |_| {
        let func: Option<Box<dyn Fn(DynamicImage, RgbColor) -> Result<RgbaImage, ServerFnError>>> =
            match selected_effect.read().as_str() {
                "or" => Some(Box::new(or)),
                "and" => Some(Box::new(and)),
                "xor" => Some(Box::new(xor)),
                _ => None,
            };

        if let Some(image) = uploaded_image.read().clone() {
            is_processing.set(true);
            upload_status.set("Processing image with OR function...".to_string());

            spawn(async move {
                // Apply your OR function
                let rgba_image = if let Some(f) = func {
                    f(image, *color.read())
                } else {
                    // Handle the case where no function is selected
                    // You could return the original image converted to RgbaImage or show an error
                    upload_status.set("No effect selected".to_string());
                    is_processing.set(false);
                    return; // or continue, depending on your control flow
                };

                // Convert to data URL for display
                match rgba_image_to_data_url(&rgba_image.unwrap()) {
                    Ok(data_url) => {
                        processed_image_url.set(Some(data_url));
                        upload_status.set("Image processed successfully!".to_string());
                    }
                    Err(e) => {
                        upload_status.set(format!("Error processing image: {}", e));
                    }
                }
                is_processing.set(false);
            });
        }
    };

    // Clear all images
    let clear_images = move |_| {
        uploaded_image.set(None);
        original_image_url.set(None);
        processed_image_url.set(None);
        upload_status.set(String::new());
        is_processing.set(false);
    };

    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            style: "max-width: 1200px; margin: 0 auto; padding: 20px; font-family: Arial, sans-serif;",

            h1 {
                style: "text-align: center; color: #333;",
                "Image Processing with OR Function"
            }

            // Upload section
            div {
                style: "margin-bottom: 30px; padding: 20px; border: 2px dashed #ccc; border-radius: 8px; background-color: #000000;",

                label {
                    style: "display: block; margin-bottom: 10px; font-weight: bold; font-size: 16px;",
                    "Select an image file:"
                }

                input {
                    r#type: "file",
                    accept: "image/*",
                    onchange: handle_file_upload,
                    style: "width: 100%; padding: 10px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;",
                }
            }

            // Status message
            if !upload_status.read().is_empty() {
                div {
                    style: "padding: 15px; margin-bottom: 20px; border-radius: 5px; background-color: #000000; border: 1px solid #bee5eb;",
                    "{upload_status.read()}"
                }
            }

            ul {
                EffectItem { title: "Item 1" }
                EffectItem { title: "Item 2" }
                EffectItem { title: "Item 3" }
            }

            div {
                style: "display: flex; flex-direction: row; justify-content: space-between; width: 100%; padding: 4px;",

                div {
                    select {
                        value: "{selected_effect}",
                        onchange: move |evt: FormEvent| selected_effect.set(evt.data.value()),
                        option { value: "or", "Bitwise OR" }
                        option { value: "and", "Bitwise AND" }
                        option { value: "xor", "Bitwise XOR" }
                    }
                }

                div {
                    ColorPicker { color: color }
                }
            }

            // Action buttons
            if uploaded_image.read().is_some() {
                div {
                    style: "text-align: center; margin-bottom: 30px;",

                    button {
                        onclick: apply_or_function,
                        disabled: *is_processing.read(),
                        style: "margin-right: 10px; padding: 12px 24px; background-color: #000000; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px;",
                        if *is_processing.read() { "Processing..." } else { "Apply OR Function" }
                    }

                    button {
                        onclick: clear_images,
                        style: "padding: 12px 24px; background-color: #000000; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px;",
                        "Clear All"
                    }
                }
            }

            // Images display section
            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 20px;",

                // Original image
                if let Some(original_url) = original_image_url.read().as_ref() {
                    div {
                        style: "border: 1px solid #ddd; padding: 20px; border-radius: 8px; background-color: #000000;",

                        h3 {
                            style: "text-align: center; margin-bottom: 15px; color: #333;",
                            "Original Image"
                        }

                        div {
                            style: "text-align: center;",
                            img {
                                src: "{original_url}",
                                style: "max-width: 100%; max-height: 400px; border: 1px solid #ccc; border-radius: 4px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                                alt: "Original uploaded image",
                            }
                        }
                    }
                }

                // Processed image
                if let Some(processed_url) = processed_image_url.read().as_ref() {
                    div {
                        style: "border: 1px solid #ddd; padding: 20px; border-radius: 8px; background-color: #000000;",

                        h3 {
                            style: "text-align: center; margin-bottom: 15px; color: #333;",
                            "Processed Image (OR Function Applied)"
                        }

                        div {
                            style: "text-align: center;",
                            img {
                                src: "{processed_url}",
                                style: "max-width: 100%; max-height: 400px; border: 1px solid #ccc; border-radius: 4px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                                alt: "Processed image with OR function applied",
                            }
                        }
                    }
                }
            }

            // Instructions when no image is uploaded
            if uploaded_image.read().is_none() {
                div {
                    style: "text-align: center; padding: 40px; color: #666; font-style: italic;",
                    "Upload an image to get started!"
                }
            }
        }

        // Router::<Route>{}
    }
}
