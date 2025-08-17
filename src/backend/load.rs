use dioxus::{
    logger::tracing::{info, Event},
    prelude::*,
};

#[server]
pub async fn load_image() -> Result<(), ServerFnError> {
    Ok(())
}
