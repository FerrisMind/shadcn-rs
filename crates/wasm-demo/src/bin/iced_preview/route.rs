#[cfg(target_arch = "wasm32")]
use super::app::ComponentTab;
#[cfg(target_arch = "wasm32")]
use super::catalog::PreviewPage;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
pub fn parse_query(search: &str) -> (Option<PreviewPage>, Option<ComponentTab>) {
    let raw = search.strip_prefix('?').unwrap_or(search);
    let mut component = None;
    let mut tab = None;

    for part in raw.split('&') {
        if part.is_empty() {
            continue;
        }
        let mut kv = part.splitn(2, '=');
        let key = kv.next().unwrap_or_default();
        let value = kv.next().unwrap_or_default();
        match key {
            "component" => component = PreviewPage::from_slug(value),
            "tab" => tab = ComponentTab::from_slug(value),
            _ => {}
        }
    }

    (component, tab)
}

#[cfg(target_arch = "wasm32")]
pub fn read_initial_route() -> (PreviewPage, ComponentTab) {
    let Some(window) = web_sys::window() else {
        return (PreviewPage::Home, ComponentTab::Demo);
    };
    let Ok(search) = window.location().search() else {
        return (PreviewPage::Home, ComponentTab::Demo);
    };

    let (component, tab) = parse_query(&search);
    (
        component.unwrap_or(PreviewPage::Home),
        tab.unwrap_or(ComponentTab::Demo),
    )
}

#[cfg(target_arch = "wasm32")]
pub fn sync_url(selected: PreviewPage, tab: ComponentTab) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let location = window.location();
    let pathname = location.pathname().unwrap_or_else(|_| "/".to_owned());
    let wanted_query = format!("?component={}&tab={}", selected.slug(), tab.slug());
    let current_query = location.search().unwrap_or_default();
    if current_query == wanted_query {
        return;
    }

    let url = format!("{pathname}{wanted_query}");
    if let Ok(history) = window.history() {
        let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&url));
    }
}
