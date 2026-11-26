//! Dioxus components implementing the Graphite UI HTML contracts.

use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use wasm_bindgen::JsCast;

/// Shared stylesheet for Dioxus apps. Keep the source token and component CSS
/// files alongside the JavaScript package so both render the same styles.
pub const STYLESHEET: &str = concat!(
    include_str!("../../web/tokens/graphite-tokens.css"),
    "\n",
    include_str!("../../web/ui/components.css"),
);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Ghost,
    Danger,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavigationItem {
    pub id: String,
    pub label: String,
    pub href: String,
}

impl ButtonVariant {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Ghost => "ghost",
            Self::Danger => "danger",
        }
    }
}

#[component]
pub fn Button(variant: ButtonVariant, onclick: EventHandler<()>, children: Element) -> Element {
    rsx! {
        button { class: "gr-button", "data-variant": variant.as_str(), r#type: "button", onclick: move |_| onclick.call(()), {children} }
    }
}

/// Accessible field wrapper. Help and error descriptions are siblings of the
/// label and control, matching the `web/ui` contract.
#[component]
pub fn Field(
    id: String,
    label: String,
    value: Option<String>,
    input_type: Option<String>,
    help: Option<String>,
    error: Option<String>,
    oninput: EventHandler<String>,
) -> Element {
    let invalid = error.is_some();
    let help_id = format!("{id}-help");
    let error_id = format!("{id}-error");
    let described_by = match (&help, &error) {
        (Some(_), Some(_)) => Some(format!("{help_id} {error_id}")),
        (Some(_), None) => Some(help_id.clone()),
        (None, Some(_)) => Some(error_id.clone()),
        (None, None) => None,
    };
    let oninput = oninput;
    rsx! {
        div { class: "gr-field",
            label { r#for: "{id}", "{label}" }
            input {
                id: "{id}",
                class: "gr-input",
                r#type: input_type.unwrap_or_else(|| "text".to_owned()),
                value: value.unwrap_or_default(),
                "aria-invalid": if invalid { Some("true") } else { None },
                "aria-describedby": described_by,
                oninput: move |event| oninput.call(event.value()),
            }
            if let Some(help) = help {
                p { id: "{help_id}", class: "gr-field-help", "{help}" }
            }
            if let Some(error) = error {
                p { id: "{error_id}", class: "gr-field-error", "{error}" }
            }
        }
    }
}

/// Native confirmation dialog. The caller controls `open`; native `showModal`
/// supplies the browser focus trap and Escape behavior when rendered in a web app.
#[component]
pub fn ConfirmDialog(
    id: String,
    message: String,
    open: bool,
    confirm_label: Option<String>,
    cancel_label: Option<String>,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    let title_id = format!("{id}-message");
    let effect_id = id.clone();
    use_effect(use_reactive!(|open| {
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        let Some(element) = document.get_element_by_id(&effect_id) else {
            return;
        };
        let Ok(dialog) = element.dyn_into::<web_sys::HtmlDialogElement>() else {
            return;
        };
        if open && !dialog.open() {
            let _ = dialog.show_modal();
        } else if !open && dialog.open() {
            dialog.close();
        }
    }));
    let confirm_label = confirm_label.unwrap_or_else(|| "OK".to_owned());
    let cancel_label = cancel_label.unwrap_or_else(|| "Cancel".to_owned());
    let cancel_for_key = on_cancel.clone();
    let cancel_for_button = on_cancel;
    let confirm_for_button = on_confirm;
    rsx! {
        dialog { id: "{id}", class: "gr-dialog", "aria-labelledby": "{title_id}", onkeydown: move |event| if event.key() == Key::Escape { cancel_for_key.call(()) },
            p { id: "{title_id}", "{message}" }
            div { class: "gr-dialog-actions",
                Button { variant: ButtonVariant::Ghost, onclick: move |_| cancel_for_button.call(()), "{cancel_label}" }
                Button { variant: ButtonVariant::Primary, onclick: move |_| confirm_for_button.call(()), "{confirm_label}" }
            }
        }
    }
}

#[component]
pub fn SideRail(items: Vec<NavigationItem>, current_id: String) -> Element {
    rsx! {
        nav { class: "gr-side-rail", "aria-label": "Primary",
            for item in items {
                a {
                    class: "gr-rail-link",
                    href: "{item.href}",
                    "aria-current": if item.id == current_id { Some("page") } else { None },
                    "{item.label}"
                }
            }
        }
    }
}

#[component]
pub fn RoomList(items: Vec<NavigationItem>, current_id: String) -> Element {
    rsx! {
        ul { class: "gr-room-list",
            for item in items {
                li {
                    a {
                        class: "gr-room-link",
                        href: "{item.href}",
                        "aria-current": if item.id == current_id { Some("page") } else { None },
                        "{item.label}"
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconName {
    Paperclip,
    RailCollapse,
    RailExpand,
}

const THEMES: &[(&str, &str)] = &[
    ("graphite", "Graphite"),
    ("indigo", "Indigo"),
    ("mono-slate", "Slate"),
    ("forest-mint", "Forest Mint"),
    ("mono-sand", "Sand"),
    ("analog-azure", "Azure Tide"),
    ("midnight-indigo", "Midnight Indigo"),
    ("comp-coral", "Mint Coral"),
];

#[component]
pub fn ThemeMenu(
    selected: String,
    selected_scheme: Option<String>,
    on_change: EventHandler<String>,
    on_scheme_change: EventHandler<Option<String>>,
) -> Element {
    let close_on_escape = move |event: Event<KeyboardData>| {
        if event.key() != Key::Escape {
            return;
        }
        event.prevent_default();
        let Some(element) = event
            .data()
            .as_web_event()
            .target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
        else {
            return;
        };
        let Ok(Some(details)) = element.closest("details") else {
            return;
        };
        let _ = details.remove_attribute("open");
        if let Ok(Some(summary)) = details.query_selector("summary") {
            if let Ok(summary) = summary.dyn_into::<web_sys::HtmlElement>() {
                let _ = summary.focus();
            }
        }
    };
    rsx! {
        details { class: "gr-menu gr-theme-menu", onkeydown: close_on_escape,
            summary { class: "gr-theme-menu-button", "Theme" }
            fieldset { class: "gr-theme-panel",
                legend { "Theme" }
                for (id, name) in THEMES {
                    label { class: "gr-theme-option",
                        input {
                            r#type: "radio",
                            name: "gr-theme",
                            value: *id,
                            checked: selected == *id,
                            onchange: move |_| on_change.call((*id).to_owned()),
                        }
                        span { class: "gr-theme-swatch", "data-swatch": *id, "aria-hidden": "true" }
                        span { if *id == "indigo" { "{name} (default)" } else { "{name}" } }
                    }
                }
                fieldset { class: "gr-theme-schemes",
                    legend { "Color scheme" }
                    for (id, label, scheme) in [("system", "System", None), ("light", "Light", Some("light")), ("dark", "Dark", Some("dark"))] {
                        label { class: "gr-theme-option",
                            input {
                                r#type: "radio",
                                name: "gr-scheme",
                                value: id,
                                checked: selected_scheme == scheme.map(str::to_owned),
                                onchange: move |_| on_scheme_change.call(scheme.map(str::to_owned)),
                            }
                            "{label}"
                        }
                    }
                }
                p { class: "gr-theme-note", "Light and dark follow your device." }
            }
        }
    }
}

#[component]
pub fn Icon(name: IconName) -> Element {
    let segments: &[&str] = match name {
        IconName::Paperclip => &["M21.4 11.1 12.2 20.3a6 6 0 0 1-8.5-8.5l9.2-9.2a4 4 0 0 1 5.7 5.7l-9.2 9.2a2 2 0 0 1-2.8-2.8l8.5-8.5"],
        IconName::RailCollapse => &["M4 4h16v16H4z", "M9 4v16", "M16 9l-3 3 3 3"],
        IconName::RailExpand => &["M4 4h16v16H4z", "M9 4v16", "M13 9l3 3-3 3"],
    };
    rsx! {
        svg { class: "gr-icon", view_box: "0 0 24 24", "aria-hidden": "true", "focusable": "false",
            for segment in segments { path { d: *segment } }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_matches_shared_markup_contract() {
        fn app() -> Element {
            rsx! { Button { variant: ButtonVariant::Danger, onclick: |_| {}, "Delete" } }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("class=\"gr-button\""));
        assert!(html.contains("data-variant=\"danger\""));
        assert!(html.contains("Delete"));
    }

    #[test]
    fn field_keeps_label_text_separate_and_describes_invalid_state() {
        fn app() -> Element {
            rsx! {
                Field { id: "email", label: "Email", help: Some("Enter a work address.".to_owned()), error: Some("Use your company domain.".to_owned()), oninput: |_| {},
                }
            }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("<label for=\"email\">Email</label>"));
        assert!(html.contains("aria-describedby=\"email-help email-error\""));
        assert!(html.contains("id=\"email-error\""));
    }

    #[test]
    fn confirmation_dialog_uses_native_dialog_markup() {
        fn app() -> Element {
            rsx! { ConfirmDialog { id: "confirm", message: "Continue?", open: true, on_confirm: |_| {}, on_cancel: |_| {} } }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("<dialog id=\"confirm\" class=\"gr-dialog\""));
        assert!(html.contains("aria-labelledby=\"confirm-message\""));
        assert!(html.contains("Continue?"));
    }

    #[test]
    fn navigation_and_theme_markup_share_the_web_contract() {
        fn app() -> Element {
            let items = vec![NavigationItem {
                id: "home".into(),
                label: "Home".into(),
                href: "/".into(),
            }];
            rsx! {
                SideRail { items: items.clone(), current_id: "home" }
                RoomList { items, current_id: "home" }
                ThemeMenu { selected: "indigo", selected_scheme: None, on_change: |_| {}, on_scheme_change: |_| {} }
            }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("class=\"gr-side-rail\""));
        assert!(html.contains("aria-current=\"page\""));
        assert!(html.contains("class=\"gr-room-list\""));
        assert!(html.contains("class=\"gr-menu gr-theme-menu\""));
        assert!(html.contains("name=\"gr-theme\""));
    }

    #[test]
    fn icon_is_hidden_from_assistive_technology() {
        let html = dioxus_ssr::render_element(rsx! { Icon { name: IconName::RailCollapse } });
        assert!(html.contains("class=\"gr-icon\""));
        assert!(html.contains("aria-hidden=\"true\""));
    }
}
