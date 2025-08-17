use dioxus::prelude::*;

#[component]
pub fn List() -> Element {
    rsx! {
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            // Attributes should be defined in the element before any children
            id: "hero",
            // After all attributes are defined, we can define child elements and components
            div { id: "list_controls",
                  button {
                      "+",
                  }
            }
        }
    }
}
