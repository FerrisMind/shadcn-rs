use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use iced::{Task, window};
use wry::dpi::{LogicalPosition, LogicalSize};
use wry::{PageLoadEvent, Rect, WebView, WebViewBuilder};

use super::{
    WebPreviewBackendEvent, WebPreviewBounds, WebPreviewConsoleEntry, WebPreviewConsoleLevel,
    WebPreviewEffect,
};

const IPC_PREFIX: &str = "__wp__";
const IPC_SEP: char = '\u{1f}';
const EVENT_QUEUE_LIMIT: usize = 2048;

thread_local! {
    static PREVIEWS: RefCell<HashMap<window::Id, PreviewState>> = RefCell::new(HashMap::new());
}

struct PreviewState {
    webview: WebView,
}

fn event_queue() -> &'static Mutex<VecDeque<WebPreviewBackendEvent>> {
    static QUEUE: OnceLock<Mutex<VecDeque<WebPreviewBackendEvent>>> = OnceLock::new();
    QUEUE.get_or_init(|| Mutex::new(VecDeque::new()))
}

fn push_event(event: WebPreviewBackendEvent) {
    let mut queue = event_queue()
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    queue.push_back(event);
    while queue.len() > EVENT_QUEUE_LIMIT {
        queue.pop_front();
    }
}

pub fn drain_events() -> Vec<WebPreviewBackendEvent> {
    let mut queue = event_queue()
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    queue.drain(..).collect()
}

#[cfg(target_os = "linux")]
pub fn pump_gtk_events() {
    while gtk::events_pending() {
        gtk::main_iteration_do(false);
    }
}

#[cfg(not(target_os = "linux"))]
pub fn pump_gtk_events() {}

pub fn run(effect: WebPreviewEffect, window_id: window::Id) -> Task<()> {
    window::run(window_id, move |window| {
        apply_effect(window, window_id, effect)
    })
}

fn apply_effect(window: &dyn window::Window, window_id: window::Id, effect: WebPreviewEffect) {
    PREVIEWS.with(|slot| {
        let mut previews = slot.borrow_mut();
        match effect {
            WebPreviewEffect::Attach { url, bounds } => {
                if previews.contains_key(&window_id) {
                    if let Some(state) = previews.get(&window_id)
                        && let Err(error) = state.webview.set_bounds(to_rect(bounds))
                    {
                        push_error(format!("set bounds failed on attach: {error}"));
                    }
                    return;
                }

                let mut builder = WebViewBuilder::new()
                    .with_bounds(to_rect(bounds))
                    .with_devtools(true)
                    .with_initialization_script(WEB_PREVIEW_BOOTSTRAP_SCRIPT)
                    .with_navigation_handler(|next_url| {
                        push_event(WebPreviewBackendEvent::UrlChanged {
                            url: next_url.clone(),
                        });
                        true
                    })
                    .with_document_title_changed_handler(|title| {
                        push_event(WebPreviewBackendEvent::TitleChanged { title });
                    })
                    .with_on_page_load_handler(|event, url| match event {
                        PageLoadEvent::Started => {
                            push_event(WebPreviewBackendEvent::PageLoadStarted { url });
                        }
                        PageLoadEvent::Finished => {
                            push_event(WebPreviewBackendEvent::PageLoadFinished { url });
                        }
                    })
                    .with_ipc_handler(|request| handle_ipc_message(request.body()));

                if !url.trim().is_empty() {
                    builder = builder.with_url(url);
                }

                let window_ref = &window;
                match builder.build_as_child(window_ref) {
                    Ok(webview) => {
                        previews.insert(window_id, PreviewState { webview });
                    }
                    Err(error) => push_error(format!("attach webview failed: {error}")),
                }
            }
            WebPreviewEffect::Detach => {
                previews.remove(&window_id);
            }
            WebPreviewEffect::Navigate(url) => {
                let Some(state) = previews.get(&window_id) else {
                    push_error(String::from("navigate failed: webview is not attached"));
                    return;
                };
                if let Err(error) = state.webview.load_url(&url) {
                    push_error(format!("navigate failed: {error}"));
                }
            }
            WebPreviewEffect::Back => {
                let Some(state) = previews.get(&window_id) else {
                    push_error(String::from("back failed: webview is not attached"));
                    return;
                };
                if let Err(error) = state.webview.evaluate_script("history.back();") {
                    push_error(format!("back failed: {error}"));
                }
            }
            WebPreviewEffect::Forward => {
                let Some(state) = previews.get(&window_id) else {
                    push_error(String::from("forward failed: webview is not attached"));
                    return;
                };
                if let Err(error) = state.webview.evaluate_script("history.forward();") {
                    push_error(format!("forward failed: {error}"));
                }
            }
            WebPreviewEffect::Reload => {
                let Some(state) = previews.get(&window_id) else {
                    push_error(String::from("reload failed: webview is not attached"));
                    return;
                };
                if let Err(error) = state.webview.reload() {
                    push_error(format!("reload failed: {error}"));
                }
            }
            WebPreviewEffect::OpenInBrowser(_) => {}
            WebPreviewEffect::OpenDevTools => {
                let Some(state) = previews.get(&window_id) else {
                    push_error(String::from(
                        "open devtools failed: webview is not attached",
                    ));
                    return;
                };
                #[cfg(debug_assertions)]
                {
                    state.webview.open_devtools();
                }
                #[cfg(not(debug_assertions))]
                {
                    let _ = state;
                    push_error(String::from(
                        "open devtools unavailable in release build without wry devtools feature",
                    ));
                }
            }
            WebPreviewEffect::SetBounds(bounds) => {
                let Some(state) = previews.get(&window_id) else {
                    push_error(String::from("set bounds failed: webview is not attached"));
                    return;
                };
                if let Err(error) = state.webview.set_bounds(to_rect(bounds)) {
                    push_error(format!("set bounds failed: {error}"));
                }
            }
        }
    });
}

fn to_rect(bounds: WebPreviewBounds) -> Rect {
    let normalized = bounds.normalized();
    Rect {
        position: LogicalPosition::new(normalized.x.round() as i32, normalized.y.round() as i32)
            .into(),
        size: LogicalSize::new(
            normalized.width.round() as u32,
            normalized.height.round() as u32,
        )
        .into(),
    }
}

fn push_error(message: String) {
    push_event(WebPreviewBackendEvent::Error { message });
}

fn handle_ipc_message(payload: &str) {
    let mut parts = payload.splitn(4, IPC_SEP);
    let prefix = parts.next().unwrap_or_default();
    if prefix != IPC_PREFIX {
        return;
    }

    let kind = parts.next().unwrap_or_default();
    let first = parts.next().unwrap_or_default();
    let second = parts.next().unwrap_or_default();

    match kind {
        "console" => {
            let level = parse_console_level(first);
            push_event(WebPreviewBackendEvent::Console(
                WebPreviewConsoleEntry::new(level, second, time_label_now()),
            ));
        }
        "history" => {
            push_event(WebPreviewBackendEvent::HistoryState {
                can_go_back: parse_bool(first),
                can_go_forward: parse_bool(second),
            });
        }
        "url" => {
            if !first.is_empty() {
                push_event(WebPreviewBackendEvent::UrlChanged {
                    url: first.to_owned(),
                });
            }
            if !second.is_empty() {
                push_event(WebPreviewBackendEvent::TitleChanged {
                    title: second.to_owned(),
                });
            }
        }
        "title" if !first.is_empty() => {
            push_event(WebPreviewBackendEvent::TitleChanged {
                title: first.to_owned(),
            });
        }
        "error" if !first.is_empty() => {
            push_error(first.to_owned());
        }
        _ => {}
    }
}

fn parse_bool(value: &str) -> bool {
    matches!(value, "1" | "true" | "yes" | "on")
}

fn parse_console_level(value: &str) -> WebPreviewConsoleLevel {
    match value {
        "info" => WebPreviewConsoleLevel::Info,
        "warn" => WebPreviewConsoleLevel::Warn,
        "error" => WebPreviewConsoleLevel::Error,
        "debug" => WebPreviewConsoleLevel::Debug,
        _ => WebPreviewConsoleLevel::Log,
    }
}

fn time_label_now() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() % 86_400)
        .unwrap_or(0);
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    format!("{hours:02}:{minutes:02}:{secs:02}")
}

const WEB_PREVIEW_BOOTSTRAP_SCRIPT: &str = r#"
(() => {
  if (window.__wpBridgeInstalled) return;
  window.__wpBridgeInstalled = true;

  const PREFIX = "__wp__";
  const SEP = "\u001f";
  const post = (kind, a = "", b = "") => {
    try {
      window.ipc.postMessage([PREFIX, kind, String(a), String(b)].join(SEP));
    } catch (_) {}
  };

  const safeString = (value) => {
    if (typeof value === "string") return value;
    try {
      return JSON.stringify(value);
    } catch (_) {
      try {
        return String(value);
      } catch (_) {
        return "<unserializable>";
      }
    }
  };

  let stack = [window.location.href];
  let index = 0;

  const emitUrlAndHistory = () => {
    post("url", window.location.href, document.title || "");
    post("history", index > 0 ? "1" : "0", index < stack.length - 1 ? "1" : "0");
  };

  const recordPush = (url) => {
    stack = stack.slice(0, index + 1);
    stack.push(url);
    index = stack.length - 1;
    emitUrlAndHistory();
  };

  const recordReplace = (url) => {
    if (stack.length === 0) {
      stack = [url];
      index = 0;
    } else {
      stack[index] = url;
    }
    emitUrlAndHistory();
  };

  const alignFromLocation = () => {
    const url = window.location.href;
    if (stack[index - 1] === url) {
      index -= 1;
    } else if (stack[index + 1] === url) {
      index += 1;
    } else {
      stack = stack.slice(0, index + 1);
      stack.push(url);
      index = stack.length - 1;
    }
    emitUrlAndHistory();
  };

  const wrapHistory = (name, recorder) => {
    const original = history[name];
    if (typeof original !== "function") return;

    history[name] = function (...args) {
      const result = original.apply(this, args);
      recorder(window.location.href);
      return result;
    };
  };

  wrapHistory("pushState", recordPush);
  wrapHistory("replaceState", recordReplace);
  window.addEventListener("popstate", alignFromLocation);
  window.addEventListener("hashchange", alignFromLocation);

  const wireConsole = (level) => {
    const original = console[level];
    if (typeof original !== "function") return;
    console[level] = function (...args) {
      post("console", level, args.map(safeString).join(" "));
      return original.apply(this, args);
    };
  };

  ["log", "info", "warn", "error", "debug"].forEach(wireConsole);

  window.addEventListener("error", (event) => {
    post(
      "console",
      "error",
      `${event.message || "Unknown error"} @${event.filename || "inline"}:${event.lineno || 0}:${event.colno || 0}`
    );
  });

  window.addEventListener("unhandledrejection", (event) => {
    post("console", "error", `Unhandled rejection: ${safeString(event.reason)}`);
  });

  emitUrlAndHistory();
})();
"#;
