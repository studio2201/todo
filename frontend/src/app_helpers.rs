//! Helpers for the top-level `App` component.
//!
//! `app.rs` is a single function with a chain of inline closures
//! (load_todos, verify_submit_pin, on_logout, show_toast, ...).
//! Extracting them into free functions keeps the orchestrator
//! short enough to read end-to-end and lets each piece be unit-tested
//! in isolation.

pub fn setup_online_offline_listeners(
    show_toast: yew::Callback<(String, crate::types::ToastType)>,
    locale: yew::UseStateHandle<crate::i18n::Locale>,
) {
    use crate::types::ToastType;
    use shared_frontend::i18n::Language;
    use shared_frontend::i18n::strings::{StringKey, lookup};
    use wasm_bindgen::JsCast;

    if let Some(window) = web_sys::window() {
        let show_toast_online = show_toast.clone();
        let loc_online = locale.clone();
        let on_online =
            wasm_bindgen::prelude::Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
                let lang = Language::from_code(loc_online.to_str());
                show_toast_online.emit((
                    lookup(StringKey::StatusOnline, lang).to_string(),
                    ToastType::Success,
                ));
            });
        let _ =
            window.add_event_listener_with_callback("online", on_online.as_ref().unchecked_ref());
        on_online.forget();

        let show_toast_offline = show_toast.clone();
        let loc_offline = locale.clone();
        let on_offline =
            wasm_bindgen::prelude::Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
                let lang = Language::from_code(loc_offline.to_str());
                show_toast_offline.emit((
                    lookup(StringKey::StatusOffline, lang).to_string(),
                    ToastType::Error,
                ));
            });
        let _ =
            window.add_event_listener_with_callback("offline", on_offline.as_ref().unchecked_ref());
        on_offline.forget();
    }
}

/// Issue a `GET /api/todos` request. On 401, clear authentication;
/// otherwise parse the optimistic-concurrency envelope and update
/// the `todos`, `data_version`, `current_list`, and `authenticated`
/// state hooks. On a network error, emit a localised toast.
#[allow(clippy::too_many_arguments)]
pub fn load_todos(
    todos: yew::UseStateHandle<Option<shared_core::types::TodoLists>>,
    data_version: yew::UseStateHandle<u64>,
    current_list: yew::UseStateHandle<String>,
    authenticated: yew::UseStateHandle<bool>,
    show_toast: yew::Callback<(String, crate::types::ToastType)>,
    locale: yew::UseStateHandle<crate::i18n::Locale>,
) {
    use crate::api::{self, TodoEnvelope};
    use crate::types::ToastType;

    wasm_bindgen_futures::spawn_local(async move {
        match api::fetch_todos_raw().await {
            Ok(resp) => {
                if resp.status() == 401 {
                    authenticated.set(false);
                } else if let Ok(envelope) = resp.json::<TodoEnvelope>().await {
                    authenticated.set(true);
                    let data = envelope.lists;
                    if !data.is_empty()
                        && !data.contains_key(&*current_list)
                        && let Some(first_key) = data.keys().next()
                    {
                        current_list.set(first_key.clone());
                    }
                    data_version.set(envelope.version);
                    todos.set(Some(data));
                }
            }
            Err(_) => show_toast.emit((
                crate::i18n::translate((*locale).clone(), crate::i18n::TransKey::FailedLoadTodos),
                ToastType::Error,
            )),
        }
    });
}
