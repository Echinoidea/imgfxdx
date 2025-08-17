use clap::builder::styling::RgbColor;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ColorPickerProps {
    color: Signal<RgbColor>,
}

#[component]
pub fn ColorPicker(mut props: ColorPickerProps) -> Element {
    let current_color = *props.color.read();

    rsx! {
        div {
            class: "color-picker",
            style: "display: flex; flex-direction: column; gap: 10px; padding: 15px; border: 1px solid #ccc; border-radius: 8px; max-width: 250px;",

            // Color preview
            div {
                style: "width: 100%; height: 50px; border: 2px solid #333; border-radius: 4px; background-color: rgb({current_color.0}, {current_color.1}, {current_color.2});",
            }

            // RGB sliders
            div {
                style: "display: flex; flex-direction: column; gap: 8px;",

                // Red slider
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    label {
                        style: "min-width: 30px; font-weight: bold; color: #d32f2f;",
                        "R:"
                    }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "255",
                        value: "{current_color.0}",
                        style: "flex: 1;",
                        oninput: move |evt| {
                            if let Ok(r) = evt.value().parse::<u8>() {
                                props.color.set(RgbColor(r, current_color.1, current_color.2));
                            }
                        }
                    }
                    span {
                        style: "min-width: 35px; text-align: right; font-mono;",
                        "{current_color.0}"
                    }
                }

                // Green slider
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    label {
                        style: "min-width: 30px; font-weight: bold; color: #388e3c;",
                        "G:"
                    }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "255",
                        value: "{current_color.1}",
                        style: "flex: 1;",
                        oninput: move |evt| {
                            if let Ok(g) = evt.value().parse::<u8>() {
                                props.color.set(RgbColor(current_color.0, g, current_color.2));
                            }
                        }
                    }
                    span {
                        style: "min-width: 35px; text-align: right; font-mono;",
                        "{current_color.1}"
                    }
                }

                // Blue slider
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    label {
                        style: "min-width: 30px; font-weight: bold; color: #1976d2;",
                        "B:"
                    }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "255",
                        value: "{current_color.2}",
                        style: "flex: 1;",
                        oninput: move |evt| {
                            if let Ok(b) = evt.value().parse::<u8>() {
                                props.color.set(RgbColor(current_color.0, current_color.1, b));
                            }
                        }
                    }
                    span {
                        style: "min-width: 35px; text-align: right; font-mono;",
                        "{current_color.2}"
                    }
                }
            }

            // RGB values display
            div {
                style: "text-align: center; font-mono; color: #666; font-size: 12px;",
                "rgb({current_color.0}, {current_color.1}, {current_color.2})"
            }
        }
    }
}
