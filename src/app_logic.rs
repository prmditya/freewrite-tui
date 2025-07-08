use chrono::{DateTime, Datelike, Local, Timelike};
use std::{
    fs,
    time::{Duration, Instant},
};

// --- Import from our own internal config module ---
use crate::config::{DEFAULT_SESSION_DURATION_MINS, MAX_RECENT_FILES, OUTPUT_DIR};

// --- Application States and Enums ---
pub enum AppState {
    MainMenu,
    Freewrite,
    SessionEnd {
        final_filename: String,
        word_count: usize,
        wpm: f64,
    },
    Quitting,
    CustomDurationInput,
}

pub enum MenuItem {
    Minute5,
    Minute10,
    Minute20,
    Custom,
}

impl MenuItem {
    pub fn to_string(&self) -> String {
        match self {
            MenuItem::Minute5 => "5 minute session (Default)".to_string(),
            MenuItem::Minute10 => "10 minute session".to_string(),
            MenuItem::Minute20 => "20 minute session".to_string(),
            MenuItem::Custom => "Custom duration".to_string(),
        }
    }

    pub fn to_duration(&self) -> u64 {
        match self {
            MenuItem::Minute5 => 5,
            MenuItem::Minute10 => 10,
            MenuItem::Minute20 => 20,
            MenuItem::Custom => 0,
        }
    }
}

pub enum PanelFocus {
    Sessions,
    Recent,
}

// --- Main Application Structure ---
pub struct App {
    pub text: String,
    pub start_time: Instant,
    pub session_duration: Duration,
    pub last_save_time: Instant,
    pub current_state: AppState,

    pub menu_items: Vec<MenuItem>,
    pub selected_menu_index: usize,
    pub recent_files: Vec<String>,

    pub selected_recent_index: usize,
    pub panel_focus: PanelFocus,
    pub displayed_file_content: Option<String>,
    pub custom_duration_input_text: String,
}

impl App {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            text: String::new(),
            start_time: now,
            session_duration: Duration::from_secs(DEFAULT_SESSION_DURATION_MINS * 60),
            last_save_time: now,
            current_state: AppState::MainMenu,

            menu_items: vec![
                MenuItem::Minute5,
                MenuItem::Minute10,
                MenuItem::Minute20,
                MenuItem::Custom,
            ],
            selected_menu_index: 0,
            recent_files: Self::load_recent_files(),

            selected_recent_index: 0,
            panel_focus: PanelFocus::Sessions,
            displayed_file_content: None,
            custom_duration_input_text: String::new(),
        }
    }

    pub fn get_word_count(&self) -> usize {
        self.text
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .count()
    }

    fn save_text(&self) -> String {
        let timestamp: DateTime<Local> = Local::now();
        let filename = format!(
            "{}/{:04}-{:02}-{:02}_{:02}:{:02}_freewritesession.txt",
            OUTPUT_DIR,
            timestamp.year(),
            timestamp.month(),
            timestamp.day(),
            timestamp.hour(),
            timestamp.minute()
        );

        fs::create_dir_all(OUTPUT_DIR).expect("Failed to create output directory");
        fs::write(&filename, &self.text).expect("Failed to write to file");
        filename
    }

    fn load_recent_files() -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(OUTPUT_DIR) {
            let mut sorted_entries: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().map_or(false, |ft| ft.is_file()))
                .collect();

            sorted_entries.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

            for entry in sorted_entries.into_iter().take(MAX_RECENT_FILES) {
                if let Some(name) = entry.file_name().to_str() {
                    files.push(name.to_string());
                }
            }
        }
        if files.is_empty() {
            files.push("There's no recent session".to_string());
        }
        files
    }

    pub fn update(&mut self) {
        if let AppState::Freewrite = self.current_state {
            let elapsed_time = self.start_time.elapsed();

            if elapsed_time >= self.session_duration {
                self.end_session();
            }
        }
    }

    pub fn start_session(&mut self, duration_minutes: u64) {
        self.text = String::new();
        self.start_time = Instant::now();
        self.session_duration = Duration::from_secs(duration_minutes * 60);
        self.last_save_time = Instant::now();
        self.current_state = AppState::Freewrite;
    }

    pub fn end_session(&mut self) {
        let final_filename = self.save_text();
        let word_count = self.get_word_count();
        let actual_duration_secs = self.start_time.elapsed().as_secs();
        let wpm = if actual_duration_secs > 0 {
            (word_count as f64 / actual_duration_secs as f64) * 60.0
        } else {
            0.0
        };
        self.current_state = AppState::SessionEnd {
            final_filename,
            word_count,
            wpm,
        };
        self.recent_files = App::load_recent_files();
    }

    pub fn load_file_content_from_name(&mut self, filename: String) {
        let file_path = std::path::Path::new(OUTPUT_DIR).join(filename.clone());
        match fs::read_to_string(&file_path) {
            Ok(content) => {
                self.displayed_file_content = Some(content);
            }
            Err(e) => {
                self.displayed_file_content =
                    Some(format!("Error reading file {}: {}", filename, e));
            }
        }
    }

    pub fn reset_to_main_menu(&mut self) {
        self.current_state = AppState::MainMenu;
        self.text = String::new();
        self.recent_files = App::load_recent_files();
        self.displayed_file_content = None;
        self.panel_focus = PanelFocus::Sessions;
        self.selected_menu_index = 0;
        self.selected_recent_index = 0;
    }
}
