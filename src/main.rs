use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;
use router::Route;

mod home;
mod hooks;
mod router;

fn App(cx: Scope) -> Element {
    render! { Router::<Route> {} }
}
fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();
    log::info!("starting app");
    dioxus_web::launch(App);
}
