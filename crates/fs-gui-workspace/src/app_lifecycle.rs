// app_lifecycle.rs — App-Management: Observer Pattern
//
// Design Pattern: Observer
//   AppLifecycleObserver  — trait: any type that reacts to app open/close
//   AppLifecycleEvent     — enum: Opened / Closed / Pinned / Unpinned
//   AppLifecycleBus       — concrete impl: emits fs-session Bus payloads
//
// The DesktopShell owns a Vec<Box<dyn AppLifecycleObserver>> and notifies
// all observers on every app open/close/pin/unpin.
//
// gRPC-First rule: direct fn calls are fine here because fs-session is
// a library crate (not a separate container). When fs-session ships as
// its own container, this module adds a gRPC client in front.

use fs_session::{AppClosedPayload, AppOpenedPayload};

// ── AppLifecycleEvent ─────────────────────────────────────────────────────────

/// Events the desktop emits when an app changes lifecycle state.
#[derive(Debug, Clone)]
pub enum AppLifecycleEvent {
    /// An app window was opened (`app_id` = package id, e.g. "fs-store").
    Opened { app_id: String, session_id: String },
    /// An app window was closed.
    Closed { app_id: String, session_id: String },
    /// An app was pinned to the sidebar.
    Pinned { app_id: String },
    /// An app was unpinned from the sidebar.
    Unpinned { app_id: String },
}

// ── AppLifecycleObserver ──────────────────────────────────────────────────────

/// Any type that can react to app lifecycle events.
///
/// Implement this trait to hook into app open/close/pin events.
/// All methods have default no-op implementations — only override what you need.
pub trait AppLifecycleObserver: Send + Sync {
    fn on_event(&self, event: &AppLifecycleEvent);
}

// ── SessionLifecycleObserver ──────────────────────────────────────────────────

/// Default observer: logs lifecycle events and emits fs-session Bus payloads.
///
/// In the future this will call `SessionTracker::open_app` / `close_app`
/// via gRPC once fs-session ships as a standalone container.
/// For now it produces the typed payloads for direct logging.
pub struct SessionLifecycleObserver;

impl AppLifecycleObserver for SessionLifecycleObserver {
    fn on_event(&self, event: &AppLifecycleEvent) {
        match event {
            AppLifecycleEvent::Opened { app_id, session_id } => {
                Self::log_opened(app_id, session_id);
            }
            AppLifecycleEvent::Closed { app_id, session_id } => {
                Self::log_closed(app_id, session_id);
            }
            AppLifecycleEvent::Pinned { app_id } => {
                tracing::info!(app_id = %app_id, "app.pinned");
            }
            AppLifecycleEvent::Unpinned { app_id } => {
                tracing::info!(app_id = %app_id, "app.unpinned");
            }
        }
    }
}

impl SessionLifecycleObserver {
    fn log_opened(app_id: &str, session_id: &str) {
        let payload = AppOpenedPayload {
            session_id: session_id.to_string(),
            app_id: app_id.to_string(),
        };
        tracing::info!(
            app_id = %payload.app_id,
            session_id = %payload.session_id,
            "app.opened"
        );
    }

    fn log_closed(app_id: &str, session_id: &str) {
        let payload = AppClosedPayload {
            session_id: session_id.to_string(),
            app_id: app_id.to_string(),
        };
        tracing::info!(
            app_id = %payload.app_id,
            session_id = %payload.session_id,
            "app.closed"
        );
    }
}

// ── AppLifecycleBus ───────────────────────────────────────────────────────────

/// Manages a list of observers and dispatches lifecycle events to all of them.
#[derive(Default)]
pub struct AppLifecycleBus {
    observers: Vec<Box<dyn AppLifecycleObserver>>,
    /// Current session id — sourced from fs-session at startup.
    pub session_id: String,
}

impl AppLifecycleBus {
    /// Create a bus with the default `SessionLifecycleObserver` registered.
    #[must_use]
    pub fn with_defaults(session_id: impl Into<String>) -> Self {
        let mut bus = Self {
            observers: vec![],
            session_id: session_id.into(),
        };
        bus.register(Box::new(SessionLifecycleObserver));
        bus
    }

    /// Register an additional observer.
    pub fn register(&mut self, observer: Box<dyn AppLifecycleObserver>) {
        self.observers.push(observer);
    }

    /// Emit an event to all registered observers.
    pub fn emit(&self, event: &AppLifecycleEvent) {
        for observer in &self.observers {
            observer.on_event(event);
        }
    }

    /// Convenience: emit `Opened` for the given app.
    pub fn app_opened(&self, app_id: &str) {
        self.emit(&AppLifecycleEvent::Opened {
            app_id: app_id.to_string(),
            session_id: self.session_id.clone(),
        });
    }

    /// Convenience: emit `Closed` for the given app.
    pub fn app_closed(&self, app_id: &str) {
        self.emit(&AppLifecycleEvent::Closed {
            app_id: app_id.to_string(),
            session_id: self.session_id.clone(),
        });
    }

    /// Convenience: emit `Pinned` for the given app.
    pub fn app_pinned(&self, app_id: &str) {
        self.emit(&AppLifecycleEvent::Pinned {
            app_id: app_id.to_string(),
        });
    }

    /// Convenience: emit `Unpinned` for the given app.
    pub fn app_unpinned(&self, app_id: &str) {
        self.emit(&AppLifecycleEvent::Unpinned {
            app_id: app_id.to_string(),
        });
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct CountingObserver(Arc<Mutex<usize>>);

    impl AppLifecycleObserver for CountingObserver {
        fn on_event(&self, _event: &AppLifecycleEvent) {
            *self.0.lock().unwrap() += 1;
        }
    }

    #[test]
    fn observer_receives_opened_event() {
        let count = Arc::new(Mutex::new(0usize));
        let mut bus = AppLifecycleBus {
            observers: vec![],
            session_id: "test-session".into(),
        };
        bus.register(Box::new(CountingObserver(Arc::clone(&count))));
        bus.app_opened("fs-store");
        assert_eq!(*count.lock().unwrap(), 1);
    }

    #[test]
    fn observer_receives_pin_and_unpin() {
        let count = Arc::new(Mutex::new(0usize));
        let mut bus = AppLifecycleBus {
            observers: vec![],
            session_id: "test-session".into(),
        };
        bus.register(Box::new(CountingObserver(Arc::clone(&count))));
        bus.app_pinned("fs-store");
        bus.app_unpinned("fs-store");
        assert_eq!(*count.lock().unwrap(), 2);
    }

    #[test]
    fn bus_with_defaults_has_session_observer() {
        let bus = AppLifecycleBus::with_defaults("session-42");
        assert_eq!(bus.session_id, "session-42");
        assert_eq!(bus.observers.len(), 1);
    }
}
