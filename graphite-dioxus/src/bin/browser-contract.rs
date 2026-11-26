use dioxus::prelude::*;
use graphite_dioxus::{ConfirmDialog, ThemeMenu};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    dioxus::web::launch::launch_cfg(app, Default::default());
}

fn main() {}

#[cfg(target_arch = "wasm32")]
fn app() -> Element {
    let mut dialog_open = use_signal(|| false);
    let mut selected_theme = use_signal(|| "indigo".to_owned());
    let mut selected_scheme = use_signal(|| None::<String>);

    rsx! {
        main {
            button { id: "open-dialog", onclick: move |_| dialog_open.set(true), "Open dialog" }
            p { id: "dialog-state", "closed" }
            ConfirmDialog {
                id: "browser-confirm",
                message: "Confirm this action?",
                open: dialog_open(),
                on_confirm: move |_| dialog_open.set(false),
                on_cancel: move |_| dialog_open.set(false),
            }
            ThemeMenu {
                selected: selected_theme(),
                selected_scheme: selected_scheme(),
                on_change: move |theme| selected_theme.set(theme),
                on_scheme_change: move |scheme| selected_scheme.set(scheme),
            }
        }
    }
}
