use backend::{Effect, *};
use clap::builder::styling::RgbColor;
use components::{ColorPicker, EffectItem, NavBar};
use dioxus::prelude::*;
use image::{DynamicImage, Rgb, RgbaImage};
use imgfx::{Direction, SortBy};
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

#[derive(Props, Clone, PartialEq)]
pub struct EffectFormProps {
    effect_list: Signal<Vec<crate::backend::Effect>>,
}

#[derive(Clone, Copy, PartialEq)]
enum EffectType {
    Or,
    And,
    Xor,
    Left,
    Right,
    Add,
    Sub,
    Mult,
    Pow,
    Div,
    Average,
    Screen,
    Overlay,
    Bloom,
    Sort,
}

impl EffectType {
    fn name(&self) -> &'static str {
        match self {
            EffectType::Or => "OR",
            EffectType::And => "AND",
            EffectType::Xor => "XOR",
            EffectType::Left => "Left",
            EffectType::Right => "Right",
            EffectType::Add => "Add",
            EffectType::Sub => "Subtract",
            EffectType::Mult => "Multiply",
            EffectType::Pow => "Power",
            EffectType::Div => "Divide",
            EffectType::Average => "Average",
            EffectType::Screen => "Screen",
            EffectType::Overlay => "Overlay",
            EffectType::Bloom => "Bloom",
            EffectType::Sort => "Sort",
        }
    }

    fn accepts_color(&self) -> bool {
        matches!(
            self,
            EffectType::Or
                | EffectType::And
                | EffectType::Xor
                | EffectType::Add
                | EffectType::Sub
                | EffectType::Mult
                | EffectType::Pow
                | EffectType::Div
                | EffectType::Average
                | EffectType::Screen
                | EffectType::Overlay
        )
    }

    fn to_effect(&self, color: Rgb<u8>) -> Effect {
        match self {
            EffectType::Or => Effect::Or {
                color,
                negate: false,
            },
            EffectType::And => Effect::And {
                color,
                negate: false,
            },
            EffectType::Xor => Effect::Xor {
                color,
                negate: false,
            },
            EffectType::Add => Effect::Add { color },
            EffectType::Sub => Effect::Sub {
                color,
                negate: false,
            },
            EffectType::Mult => Effect::Mult { color },
            EffectType::Pow => Effect::Pow { color },
            EffectType::Div => Effect::Div { color },
            EffectType::Average => Effect::Average { color },
            EffectType::Screen => Effect::Screen { color },
            EffectType::Overlay => Effect::Overlay { color },
            // Non-color effects with default values
            EffectType::Left => Effect::Left {
                bits: 1,
                negate: false,
            },
            EffectType::Right => Effect::Right {
                bits: 1,
                negate: false,
            },
            EffectType::Bloom => Effect::Bloom {
                intensity: 1.0,
                radius: 5.0,
                min_threshold: 128,
                max_threshold: Some(255),
            },
            EffectType::Sort => Effect::Sort {
                direction: Direction::Horizontal, // You'll need to define this
                sort_by: SortBy::Luminance,       // You'll need to define this
                min_threshold: 0.0,
                max_threshold: 1.0,
                reversed: false,
            },
        }
    }
}

/// Form to add new effects to the list
#[component]
fn EffectForm(mut props: EffectFormProps) -> Element {
    let mut selected_effect_type = use_signal(|| None::<EffectType>);
    let selected_color = use_signal(|| Rgb([255u8, 0u8, 0u8]));

    let effect_types = vec![
        EffectType::Or,
        EffectType::And,
        EffectType::Xor,
        EffectType::Left,
        EffectType::Right,
        EffectType::Add,
        EffectType::Sub,
        EffectType::Mult,
        EffectType::Pow,
        EffectType::Div,
        EffectType::Average,
        EffectType::Screen,
        EffectType::Overlay,
        EffectType::Bloom,
        EffectType::Sort,
    ];

    let add_effect = move |_| {
        if let Some(effect_type) = *selected_effect_type.read() {
            let new_effect = effect_type.to_effect(*selected_color.read());
            let mut current_list = props.effect_list.read().clone();
            current_list.push(new_effect);
            props.effect_list.set(current_list);

            // Reset form
            // *selected_effect_type.set(None);
        }
    };

    rsx! {
        form {
            prevent_default: "onsubmit",
            onsubmit: add_effect,

            div {
                label { "Effect Type:" }
                select {
                    value: if let Some(effect_type) = selected_effect_type.read().as_ref() {
                        format!("{}", effect_types.iter().position(|e| e == effect_type).unwrap_or(0))
                    } else {
                        String::new()
                    },
                    onchange: move |evt| {
                        if let Ok(index) = evt.value().parse::<usize>() {
                            if let Some(effect_type) = effect_types.get(index) {
                                selected_effect_type.set(Some(*effect_type));
                            }
                        }
                    },
                    option { value: "", "Select an effect..." }
                    for (i, effect_type) in effect_types.iter().enumerate() {
                        option {
                            value: "{i}",
                            "{effect_type.name()}"
                        }
                    }
                }
            }

            // Show color picker only if selected effect accepts color
            if let Some(effect_type) = selected_effect_type.read().as_ref() {
                if effect_type.accepts_color() {
                    div {
                        label { "Color:" }
                        ColorPicker { color: selected_color }
                    }
                }
            }

            div {
                button {
                    r#type: "submit",
                    disabled: selected_effect_type.read().is_none(),
                    "Add Effect"
                }
            }
        }
    }
}

#[component]
fn App() -> Element {
    let mut uploaded_image = use_signal(|| None::<DynamicImage>);
    let mut original_image_url = use_signal(|| None::<String>);
    let mut processed_image_url = use_signal(|| None::<String>);
    let mut upload_status = use_signal(|| String::new());
    let mut is_processing = use_signal(|| false);
    let color = use_signal(|| Rgb([0, 0, 0]));
    let effect_list = use_signal(|| vec![]);

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
    let apply_effects = move |_| {
        if let Some(image) = uploaded_image.read().clone() {
            let effects: Vec<Effect> = effect_list.read().clone(); // Clone here instead of just borrowing
            if effects.is_empty() {
                upload_status.set("No effects in the list to apply".to_string());
                return;
            }
            is_processing.set(true);
            upload_status.set("Processing image with effect chain...".to_string());
            spawn(async move {
                let mut current_image = image;
                // Apply each effect in sequence
                for (index, effect) in effects.iter().enumerate() {
                    upload_status.set(format!(
                        "Applying effect {} of {}: {}",
                        index + 1,
                        effects.len(),
                        effect.name()
                    ));
                    match apply_single_effect(current_image.clone(), effect) {
                        Ok(processed) => {
                            // Convert RgbaImage back to DynamicImage for the next effect
                            current_image = DynamicImage::ImageRgba8(processed);
                        }
                        Err(e) => {
                            upload_status.set(format!(
                                "Error applying effect {}: {}",
                                index + 1,
                                e
                            ));
                            is_processing.set(false);
                            return;
                        }
                    }
                }
                // Convert final result to data URL for display
                let final_rgba = current_image.to_rgba8();
                match rgba_image_to_data_url(&final_rgba) {
                    Ok(data_url) => {
                        processed_image_url.set(Some(data_url));
                        upload_status
                            .set(format!("Successfully applied {} effects!", effects.len()));
                    }
                    Err(e) => {
                        upload_status.set(format!("Error creating final image: {}", e));
                    }
                }
                is_processing.set(false);
            });
        }
    };

    // Helper function to apply a single effect
    fn apply_single_effect(
        image: DynamicImage,
        effect: &crate::backend::Effect,
    ) -> Result<RgbaImage, ServerFnError> {
        match effect {
            crate::backend::Effect::Or { color, negate } => or(image, *color),
            crate::backend::Effect::And { color, negate } => and(image, *color),
            crate::backend::Effect::Xor { color, negate } => xor(image, *color),
            crate::backend::Effect::Left { bits, negate } => left(image, bits.clone()),
            crate::backend::Effect::Right { bits, negate } => right(image, bits.clone()),
            _ => or(image, Rgb([0, 0, 0])), // crate::backend::Effect::Add { color } => add(image, *color),
                                            // crate::backend::Effect::Sub { color, negate } => sub(image, *color),
                                            // crate::backend::Effect::Mult { color } => mult(image, *color),
                                            // crate::backend::Effect::Pow { color } => pow(image, *color),
                                            // crate::backend::Effect::Div { color } => div(image, *color),
                                            // crate::backend::Effect::Average { color } => average(image, *color),
                                            // crate::backend::Effect::Screen { color } => screen(image, *color),
                                            // crate::backend::Effect::Overlay { color } => overlay(image, *color),
                                            // crate::backend::Effect::Bloom {
                                            //     intensity,
                                            //     radius,
                                            //     min_threshold,
                                            //     max_threshold,
                                            // } => {
                                            //     // You'll need to create or update your bloom function to accept these parameters
                                            //     bloom_with_params(image, *intensity, *radius, *min_threshold, *max_threshold)
                                            // }
                                            // crate::backend::Effect::Sort {
                                            //     direction,
                                            //     sort_by,
                                            //     min_threshold,
                                            //     max_threshold,
                                            //     reversed,
                                            // } => {
                                            //     // You'll need to create or update your sort function to accept these parameters
                                            //     sort_with_params(
                                            //         image,
                                            //         *direction,
                                            //         *sort_by,
                                            //         *min_threshold,
                                            //         *max_threshold,
                                            //         *reversed,
                                            //     )
                                            // }
        }
    }

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
                "imgfx DIOXUS"
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

            EffectForm { effect_list: effect_list }

            ul {
                for (index, effect) in effect_list.read().iter().enumerate() {
                    EffectItem {
                        title: effect.clone().name(),
                        key: "{index}",
                        index,
                        effect: effect.clone(),
                        effect_list,
                    }
                }
            }


            // Action buttons
            if uploaded_image.read().is_some() {
                div {
                    style: "text-align: center; margin-bottom: 30px;",

                    button {
                        onclick: apply_effects,
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
