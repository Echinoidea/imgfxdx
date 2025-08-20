#![allow(non_snake_case)]

use dioxus::prelude::*;
use image::Rgb;

use crate::components::ColorPicker;

#[derive(Clone, Props, PartialEq)]
pub struct EffectItemProps {
    title: String,
    index: usize,
    effect: crate::backend::Effect,
    effect_list: Signal<Vec<crate::backend::Effect>>,
}

#[derive(Clone, Props, PartialEq)]
struct BitwiseEffectProps {
    color: Signal<Rgb<u8>>,
    negate: bool,
}

/// This needs to have its own inputs that will be checked when processing this effect
#[component]
fn BitwiseEffect(props: BitwiseEffectProps) -> Element {
    rsx! {}
}

#[component]
pub fn EffectItem(mut props: EffectItemProps) -> Element {
    let color = use_signal(|| Rgb([0, 0, 0]));

    let remove_effect = move |_| {
        let mut current_list = props.effect_list.read().clone();
        current_list.remove(props.index);
        props.effect_list.set(current_list);
    };

    let move_up = move |_| {
        let mut current_list = props.effect_list.read().clone();
        if props.index > 0 {
            current_list.swap(props.index, props.index - 1);
            props.effect_list.set(current_list);
        }
    };

    let move_down = move |_| {
        let mut current_list = props.effect_list.read().clone();
        if props.index < current_list.len() - 1 {
            current_list.swap(props.index, props.index + 1);
            props.effect_list.set(current_list);
        }
    };

    rsx! {
        ul {
            class: "effect-item",
            div {
                style: "display: flex; flex-direction: row; justify-content: space-between; width: 100%; transition: transform 0.2s;",

                p {style: "font-size: 20px; text-align: center;", "{props.title}"}

                div {
                    id: "effect-controls-move",
                    button { class: "effect-control-button", onclick: move_up, "↑" }
                    button { class: "effect-control-button", onclick: move_down, "↓" }
                }
                button {class: "effect-control-button", onclick: remove_effect, "-" }
            }
        }
    }
}
