use crate::home::Home;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Home {},
}
