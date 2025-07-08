use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use std::io;

// Import from the crate root
use crate::app_logic::{App, AppState, PanelFocus};
use crate::config::DEFAULT_SESSION_DURATION_MINS;

pub fn handle_event(
    app: &mut App,
    key_event_kind: KeyEventKind,
    key_code: KeyCode,
    key_modifiers: KeyModifiers,
) -> io::Result<()> {
    if key_event_kind == KeyEventKind::Press {
        match app.current_state {
            AppState::MainMenu => handle_main_menu_event(app, key_code)?,
            AppState::Freewrite => handle_freewrite_event(app, key_code, key_modifiers)?,
            AppState::SessionEnd { .. } => handle_session_end_event(app, key_code),
            AppState::CustomDurationInput => handle_custom_duration_input_event(app, key_code)?,
            AppState::Quitting => {}
        }
    }
    Ok(())
}

fn handle_main_menu_event(app: &mut App, key_code: KeyCode) -> io::Result<()> {
    match key_code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.current_state = AppState::Quitting;
        }
        KeyCode::Char('h') => {
            app.panel_focus = PanelFocus::Sessions;
            app.displayed_file_content = None;
        }
        KeyCode::Char('l') => {
            app.panel_focus = PanelFocus::Recent;
            if !app.recent_files.is_empty() && app.recent_files[0] != "There's no recent session." {
                let filename_to_load = app.recent_files[app.selected_recent_index].clone();
                app.load_file_content_from_name(filename_to_load);
            } else {
                app.displayed_file_content = None;
            }
        }
        KeyCode::Char('j') => match app.panel_focus {
            PanelFocus::Sessions => {
                app.selected_menu_index = (app.selected_menu_index + 1) % app.menu_items.len();
            }
            PanelFocus::Recent => {
                if !app.recent_files.is_empty()
                    && app.recent_files[0] != "There's no recent session."
                {
                    app.selected_recent_index =
                        (app.selected_recent_index + 1) % app.recent_files.len();
                    let filename_to_load = app.recent_files[app.selected_recent_index].clone();
                    app.load_file_content_from_name(filename_to_load);
                }
            }
        },
        KeyCode::Char('k') => match app.panel_focus {
            PanelFocus::Sessions => {
                app.selected_menu_index =
                    (app.selected_menu_index + app.menu_items.len() - 1) % app.menu_items.len();
            }
            PanelFocus::Recent => {
                if !app.recent_files.is_empty()
                    && app.recent_files[0] != "There's no recent session."
                {
                    app.selected_recent_index =
                        (app.selected_recent_index + app.recent_files.len() - 1)
                            % app.recent_files.len();
                    let filename_to_load = app.recent_files[app.selected_recent_index].clone();
                    app.load_file_content_from_name(filename_to_load);
                }
            }
        },
        KeyCode::Enter => match app.panel_focus {
            PanelFocus::Sessions => {
                let selected_option = &app.menu_items[app.selected_menu_index];
                let duration = selected_option.to_duration();

                if duration == 0 {
                    app.custom_duration_input_text = String::new();
                    app.current_state = AppState::CustomDurationInput;
                } else {
                    app.start_session(duration);
                }
            }
            PanelFocus::Recent => {
                // Action for selecting a recent file, content is already displayed
            }
        },
        _ => {}
    }
    Ok(())
}

fn handle_freewrite_event(
    app: &mut App,
    key_code: KeyCode,
    key_modifiers: KeyModifiers,
) -> io::Result<()> {
    match key_code {
        KeyCode::Char('c') if key_modifiers.contains(KeyModifiers::CONTROL) => {
            app.current_state = AppState::Quitting;
        }
        KeyCode::Esc => {
            app.end_session();
            app.reset_to_main_menu();
        }
        KeyCode::Backspace => {
            if !app.text.is_empty() {
                app.text.pop();
            }
        }
        KeyCode::Enter => {
            app.text.push('\n');
        }
        KeyCode::Char(c) => {
            if !c.is_control() {
                app.text.push(c);
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_session_end_event(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Enter => {
            app.reset_to_main_menu();
        }
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.current_state = AppState::Quitting;
        }
        _ => {}
    }
}

fn handle_custom_duration_input_event(app: &mut App, key_code: KeyCode) -> io::Result<()> {
    match key_code {
        KeyCode::Enter => {
            // Try to parse the input as a number
            let custom_duration: u64 = app
                .custom_duration_input_text
                .trim()
                .parse()
                .unwrap_or(DEFAULT_SESSION_DURATION_MINS); // Fallback to default if parse fails

            app.start_session(custom_duration);
            app.custom_duration_input_text.clear(); // Clear input field
        }
        KeyCode::Esc => {
            // Cancel input, return to main menu
            app.current_state = AppState::MainMenu;
            app.custom_duration_input_text.clear(); // Clear input field
        }
        KeyCode::Backspace => {
            app.custom_duration_input_text.pop();
        }
        KeyCode::Char(c) => {
            // Only allow digits
            if c.is_ascii_digit() {
                app.custom_duration_input_text.push(c);
            }
        }
        _ => {} // Ignore other keys
    }
    Ok(())
}
