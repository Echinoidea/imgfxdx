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

    rsx! {
        ul {
            div {
                style: "display: flex; flex-direction: row; justify-content: space-between; width: 100%;",

                p {"{props.title}"}

                button {onclick: remove_effect, "-" }
            }
        }
    }
}
