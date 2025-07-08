use ratatui::style::Color;

// --- General Configuration ---
pub const DEFAULT_SESSION_DURATION_MINS: u64 = 5;
pub const OUTPUT_DIR: &str = "freewrite_sessions";
pub const MAX_RECENT_FILES: usize = 10;

// --- Color Palette ---
pub const NORMAL_BORDER_COLOR: Color = Color::DarkGray;
pub const FOCUS_BORDER_COLOR: Color = Color::LightBlue;
pub const SELECTED_ITEM_BG: Color = Color::DarkGray;
pub const SELECTED_ITEM_FG: Color = Color::White;
pub const TITLE_COLOR: Color = Color::White;
pub const ACCENT_COLOR: Color = Color::LightCyan;
pub const INFO_COLOR: Color = Color::Gray;
pub const SUCCESS_COLOR: Color = Color::Green;
pub const FILE_ITEM_COLOR: Color = Color::Green;
