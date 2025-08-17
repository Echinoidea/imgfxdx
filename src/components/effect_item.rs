use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
pub struct EffectItemProps {
    title: String,
    // effect_list: Signal<Vec<>>
}

#[component]
pub fn EffectItem(mut props: EffectItemProps) -> Element {
    rsx! {
        ul {
            div {
                style: "display: flex; flex-direction: row; justify-content: space-between; width: 100%;",
                "{props.title}",

                div {
                    style: "display: flex; flex-direction: row; justify-content: flex-end; width: 50%;",
                    button { "+" }
                    button { "-" }
                }

            }
        }
    }
}
