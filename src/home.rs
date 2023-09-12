use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use uuid::Uuid;

use crate::hooks::{use_persistent, UsePersistent};

pub fn Home(cx: Scope) -> Element {
    let uuid_persistent_state: &UsePersistent<Uuid> = use_persistent(cx, "uuid");
    let uuid = uuid_persistent_state.get();
    let new_uuid = Uuid::new_v4();
    render! {
        div { "Saved uuid: ", uuid.to_string() }
        div { "Uuid next page: ", new_uuid.to_string() }
        Link { to: "https://dioxuslabs.com", onclick: move |_| { uuid_persistent_state.set(new_uuid) }, "Update" }
    }
}
