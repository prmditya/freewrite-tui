// --- Declare internal modules ---
// These modules are now *part of* the 'freewriter_tui' library crate.
// They will be accessible as `freewriter_tui::config`, `freewriter_rs::event`, etc.
pub mod app_logic;
pub mod config;
pub mod event;
pub mod ui;

pub use app_logic::{App, AppState, MenuItem, PanelFocus};
