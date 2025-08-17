use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "title",
              Link { to: Route::Home,
                     h1 { "ImgFX" }
              }
              // Link { to: Route::Favorites, id: "heart", "♥️" }
        }
        Outlet::<Route> {}
    }
}
