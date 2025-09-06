use dioxus::prelude::*;
use routes::Home;

mod backend;
mod components;
mod routes;
mod utils;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[derive(Routable, Clone, Debug, PartialEq)]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home,
}

#[component]
fn Layout() -> Element {
    rsx! {
        Outlet::<Route> {  }
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route>{}
    }
}
