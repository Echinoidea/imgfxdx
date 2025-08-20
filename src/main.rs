use backend::{Effect, *};
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

    // CLAUDE: These functions should be used in order to create the necessary inputs in the EffectForm appropriate for the selected effect
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

    fn accepts_bits(&self) -> bool {
        matches!(self, EffectType::Left | EffectType::Right)
    }

    fn accepts_sort_by(&self) -> bool {
        matches!(self, EffectType::Sort)
    }

    fn accepts_direction(&self) -> bool {
        matches!(self, EffectType::Sort)
    }

    fn accepts_threshold(&self) -> bool {
        matches!(self, EffectType::Sort | EffectType::Bloom)
    }

    fn accepts_negate(&self) -> bool {
        matches!(
            self,
            EffectType::Or
                | EffectType::And
                | EffectType::Xor
                | EffectType::Left
                | EffectType::Right
        )
    }

    fn accepts_blur_radius(&self) -> bool {
        matches!(self, EffectType::Bloom)
    }

    fn accepts_intensity(&self) -> bool {
        matches!(self, EffectType::Bloom)
    }

    fn accepts_reversed(&self) -> bool {
        matches!(self, EffectType::Sort)
    }

    fn to_effect(&self, color: Option<Rgb<u8>>, negate: Option<bool>) -> Effect {
        match self {
            EffectType::Or => Effect::Or {
                color: color.unwrap(),
                negate: negate.unwrap(),
            },
            EffectType::And => Effect::And {
                color: color.unwrap(),
                negate: false,
            },
            EffectType::Xor => Effect::Xor {
                color: color.unwrap(),
                negate: false,
            },
            EffectType::Add => Effect::Add {
                color: color.unwrap(),
            },
            EffectType::Sub => Effect::Sub {
                color: color.unwrap(),
                negate: false,
            },
            EffectType::Mult => Effect::Mult {
                color: color.unwrap(),
            },
            EffectType::Pow => Effect::Pow {
                color: color.unwrap(),
            },
            EffectType::Div => Effect::Div {
                color: color.unwrap(),
            },
            EffectType::Average => Effect::Average {
                color: color.unwrap(),
            },
            EffectType::Screen => Effect::Screen {
                color: color.unwrap(),
            },
            EffectType::Overlay => Effect::Overlay {
                color: color.unwrap(),
            },
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
                sort_by: SortBy::Red,             // You'll need to define this
                min_threshold: 0.0,
                max_threshold: 255.0,
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
    let mut negated = use_signal(|| false);
    let mut bits = use_signal(|| 1u8);
    let mut intensity = use_signal(|| 1.0f64);
    let mut blur_radius = use_signal(|| 5.0f64);
    let mut min_threshold = use_signal(|| 128u8);
    let mut max_threshold = use_signal(|| Some(255u8));
    let mut sort_by = use_signal(|| SortBy::Red);
    let mut direction = use_signal(|| Direction::Horizontal);
    let mut sort_min_threshold = use_signal(|| 0.0f64);
    let mut sort_max_threshold = use_signal(|| 255.0f64);
    let mut reversed = use_signal(|| false);

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
            let new_effect = match effect_type {
                EffectType::Or => Effect::Or {
                    color: *selected_color.read(),
                    negate: *negated.read(),
                },
                EffectType::And => Effect::And {
                    color: *selected_color.read(),
                    negate: *negated.read(),
                },
                EffectType::Xor => Effect::Xor {
                    color: *selected_color.read(),
                    negate: *negated.read(),
                },
                EffectType::Left => Effect::Left {
                    bits: *bits.read(),
                    negate: *negated.read(),
                },
                EffectType::Right => Effect::Right {
                    bits: *bits.read(),
                    negate: *negated.read(),
                },
                EffectType::Add => Effect::Add {
                    color: *selected_color.read(),
                },
                EffectType::Sub => Effect::Sub {
                    color: *selected_color.read(),
                    negate: *negated.read(),
                },
                EffectType::Mult => Effect::Mult {
                    color: *selected_color.read(),
                },
                EffectType::Pow => Effect::Pow {
                    color: *selected_color.read(),
                },
                EffectType::Div => Effect::Div {
                    color: *selected_color.read(),
                },
                EffectType::Average => Effect::Average {
                    color: *selected_color.read(),
                },
                EffectType::Screen => Effect::Screen {
                    color: *selected_color.read(),
                },
                EffectType::Overlay => Effect::Overlay {
                    color: *selected_color.read(),
                },
                EffectType::Bloom => Effect::Bloom {
                    intensity: *intensity.read(),
                    radius: *blur_radius.read(),
                    min_threshold: *min_threshold.read(),
                    max_threshold: *max_threshold.read(),
                },
                EffectType::Sort => Effect::Sort {
                    direction: *direction.read(),
                    sort_by: *sort_by.read(),
                    min_threshold: *sort_min_threshold.read(),
                    max_threshold: *sort_max_threshold.read(),
                    reversed: *reversed.read(),
                },
            };

            let mut current_list = props.effect_list.read().clone();
            current_list.push(new_effect);
            props.effect_list.set(current_list);
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

            // Show inputs based on what the selected effect accepts
            if let Some(effect_type) = selected_effect_type.read().as_ref() {

                // Color picker
                if effect_type.accepts_color() {
                    div {
                        style: "display: flex; align-items: center; justify-content: center; width: 100%; margin: 10px 0;",
                        ColorPicker { color: selected_color }
                    }
                }

                // Negate checkbox
                if effect_type.accepts_negate() {
                    div {
                        style: "margin: 10px 0;",
                        label {
                            style: "display: flex; align-items: center; gap: 5px;",
                            input {
                                r#type: "checkbox",
                                checked: *negated.read(),
                                onchange: move |evt| {
                                    negated.set(evt.checked());
                                }
                            }
                            "Negate"
                        }
                    }
                }

                // Bits input for Left/Right shift
                if effect_type.accepts_bits() {
                    div {
                        style: "margin: 10px 0;",
                        label { "Bits:" }
                        input {
                            r#type: "number",
                            min: "1",
                            max: "8",
                            value: "{bits.read()}",
                            onchange: move |evt| {
                                if let Ok(val) = evt.value().parse::<u8>() {
                                    bits.set(val);
                                }
                            }
                        }
                    }
                }

                // Bloom-specific inputs
                if *effect_type == EffectType::Bloom {
                    div {
                        style: "margin: 10px 0;",
                        label { "Intensity:" }
                        input {
                            r#type: "number",
                            step: "0.1",
                            min: "0.1",
                            value: "{intensity.read()}",
                            onchange: move |evt| {
                                if let Ok(val) = evt.value().parse::<f64>() {
                                    intensity.set(val);
                                }
                            }
                        }
                    }

                    div {
                        style: "margin: 10px 0;",
                        label { "Blur Radius:" }
                        input {
                            r#type: "number",
                            step: "0.1",
                            min: "0.1",
                            value: "{blur_radius.read()}",
                            onchange: move |evt| {
                                if let Ok(val) = evt.value().parse::<f64>() {
                                    blur_radius.set(val);
                                }
                            }
                        }
                    }

                    div {
                        style: "margin: 10px 0;",
                        label { "Min Threshold:" }
                        input {
                            r#type: "number",
                            min: "0",
                            max: "255",
                            value: "{min_threshold.read()}",
                            onchange: move |evt| {
                                if let Ok(val) = evt.value().parse::<u8>() {
                                    min_threshold.set(val);
                                }
                            }
                        }
                    }

                    div {
                        style: "margin: 10px 0;",
                        label {
                            style: "display: flex; align-items: center; gap: 5px;",
                            input {
                                r#type: "checkbox",
                                checked: max_threshold.read().is_some(),
                                onchange: move |evt| {
                                    if evt.checked() {
                                        max_threshold.set(Some(255));
                                    } else {
                                        max_threshold.set(None);
                                    }
                                }
                            }
                            "Use Max Threshold"
                        }
                        if max_threshold.read().is_some() {
                            input {
                                r#type: "number",
                                min: "0",
                                max: "255",
                                value: "{max_threshold.read().unwrap_or(255)}",
                                onchange: move |evt| {
                                    if let Ok(val) = evt.value().parse::<u8>() {
                                        max_threshold.set(Some(val));
                                    }
                                }
                            }
                        }
                    }
                }

                // Sort-specific inputs
                if *effect_type == EffectType::Sort {
                    div {
                        style: "margin: 10px 0;",
                        label { "Sort By:" }
                        select {
                            value: match *sort_by.read() {
                                SortBy::Red=>"0",
                                SortBy::Green=>"1",
                                SortBy::Blue=>"2",
                                SortBy::Hue=>"3",
                                SortBy::Saturation=>"4",
                                SortBy::Luminance=>"5",
                                SortBy::Value => "6"
                            },
                            onchange: move |evt| {
                                match evt.value().as_str() {
                                    "0" => sort_by.set(SortBy::Red),
                                    "1" => sort_by.set(SortBy::Green),
                                    "2" => sort_by.set(SortBy::Blue),
                                    "3" => sort_by.set(SortBy::Hue),
                                    "4" => sort_by.set(SortBy::Saturation),
                                    "5" => sort_by.set(SortBy::Luminance),
                                    "6" => sort_by.set(SortBy::Value),
                                    _ => {}
                                }
                            },
                            option { value: "0", "Red" }
                            option { value: "1", "Green" }
                            option { value: "2", "Blue" }
                            option { value: "3", "Hue" }
                            option { value: "4", "Saturation" }
                            option { value: "5", "Luminance" }
                            option { value: "6", "Value" }
                        }
                    }

                    div {
                        style: "margin: 10px 0;",
                        label { "Direction:" }
                        select {
                            value: match *direction.read() {
                                Direction::Horizontal => "0",
                                Direction::Vertical => "1",
                            },
                            onchange: move |evt| {
                                match evt.value().as_str() {
                                    "0" => direction.set(Direction::Horizontal),
                                    "1" => direction.set(Direction::Vertical),
                                    _ => {}
                                }
                            },
                            option { value: "0", "Horizontal" }
                            option { value: "1", "Vertical" }
                        }
                    }

                    div {
                        style: "margin: 10px 0;",
                        label { "Min Threshold:" }
                        input {
                            r#type: "number",
                            step: "0.1",
                            min: "0",
                            max: "255",
                            value: "{sort_min_threshold.read()}",
                            onchange: move |evt| {
                                if let Ok(val) = evt.value().parse::<f64>() {
                                    sort_min_threshold.set(val);
                                }
                            }
                        }
                    }

                    div {
                        style: "margin: 10px 0;",
                        label { "Max Threshold:" }
                        input {
                            r#type: "number",
                            step: "0.1",
                            min: "0",
                            max: "255",
                            value: "{sort_max_threshold.read()}",
                            onchange: move |evt| {
                                if let Ok(val) = evt.value().parse::<f64>() {
                                    sort_max_threshold.set(val);
                                }
                            }
                        }
                    }

                    div {
                        style: "margin: 10px 0;",
                        label {
                            style: "display: flex; align-items: center; gap: 5px;",
                            input {
                                r#type: "checkbox",
                                checked: *reversed.read(),
                                onchange: move |evt| {
                                    reversed.set(evt.checked());
                                }
                            }
                            "Reversed"
                        }
                    }
                }
            }

            div {
                style: "margin: 20px 0;",
                button {
                    r#type: "submit",
                    disabled: selected_effect_type.read().is_none(),
                    style: "padding: 10px 20px; background-color: #000000; color: white; border: none; border-radius: 4px; cursor: pointer;",
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
            crate::backend::Effect::Or { color, negate } => or(image, *color, *negate),
            crate::backend::Effect::And { color, negate } => and(image, *color, *negate),
            crate::backend::Effect::Xor { color, negate } => xor(image, *color, *negate),
            crate::backend::Effect::Left { bits, negate } => left(image, bits.clone(), *negate),
            crate::backend::Effect::Right { bits, negate } => right(image, bits.clone(), *negate),
            crate::backend::Effect::Add { color } => add(image, *color),
            crate::backend::Effect::Sub { color, negate } => sub(image, *color),
            crate::backend::Effect::Mult { color } => mult(image, *color),
            crate::backend::Effect::Pow { color } => pow(image, *color),
            crate::backend::Effect::Div { color } => div(image, *color),
            crate::backend::Effect::Average { color } => average(image, *color),
            crate::backend::Effect::Screen { color } => screen(image, *color),
            crate::backend::Effect::Overlay { color } => overlay(image, *color),
            crate::backend::Effect::Bloom {
                intensity,
                radius,
                min_threshold,
                max_threshold,
            } => {
                // You'll need to create or update your bloom function to accept these parameters
                bloom(image, *intensity, *radius, *min_threshold, *max_threshold)
            }
            crate::backend::Effect::Sort {
                direction,
                sort_by,
                min_threshold,
                max_threshold,
                reversed,
            } => sort(
                image,
                *sort_by,
                *direction,
                *min_threshold,
                *max_threshold,
                *reversed,
            ),
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

            main {
                id: "main",

                // Upload section
                div {
                    id: "menu",

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



                    if !upload_status.read().is_empty() {
                        div {
                            style: "padding: 15px; margin-bottom: 20px; border-radius: 5px; background-color: #000000; border: 1px solid #bee5eb;",
                            "{upload_status.read()}"
                        }


                        // Action buttons
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 20px; justify-content:center; align-items: center;",
                        EffectForm { effect_list: effect_list }
                    }



                    div {
                        style: "display: flex; width: 100%;",
                        ul {
                            style: "width: 100%; padding-left: 0px; margin-left: 0px;",
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
                    }



                    // Buttons
                    if uploaded_image.read().is_some() {
                        div {
                            style: "text-align: center; margin-top: auto; margin-bottom: 0px;",

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
                }






                // Images display section
    div {
        class: "images-container",

        // Original image
        if let Some(original_url) = original_image_url.read().as_ref() {
            div {
                class: "image-wrapper",
                img {
                    src: "{original_url}",
                    alt: "Original image",
                }
            }
        }

        // Processed image
        if let Some(processed_url) = processed_image_url.read().as_ref() {
            div {
                class: "image-wrapper",
                img {
                    src: "{processed_url}",
                    alt: "Processed image",
                }
            }
        }

        // Instructions when no image is uploaded
        if uploaded_image.read().is_none() {
            div {
                class: "no-image-message",
                "Upload an image to get started!"
            }
        }
    }
            }

            // Router::<Route>{}
        }
}
