use chrono::Local;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::app_logic::{App, AppState, PanelFocus};
use crate::config::{
    ACCENT_COLOR, DEFAULT_SESSION_DURATION_MINS, FILE_ITEM_COLOR, FOCUS_BORDER_COLOR, INFO_COLOR,
    NORMAL_BORDER_COLOR, SELECTED_ITEM_BG, SELECTED_ITEM_FG, SUCCESS_COLOR, TITLE_COLOR,
};

// ... (rest of your UI rendering functions) ...
// The function bodies will use App, AppState, PanelFocus, and the constants directly.
// For example, instead of `Color::White`, use `SELECTED_ITEM_FG` for consistency.

pub fn ui(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let outer_margin_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    let horizontal_margin_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(outer_margin_chunks[1]);

    let inner_rect = horizontal_margin_chunks[1];

    match app.current_state {
        AppState::MainMenu => render_main_menu(frame, app, inner_rect, outer_margin_chunks[2]),
        AppState::Freewrite => {
            render_freewrite_session(frame, app, inner_rect, outer_margin_chunks[2])
        }
        AppState::SessionEnd {
            ref final_filename,
            word_count,
            wpm,
        } => {
            render_session_end(
                frame,
                final_filename,
                word_count,
                wpm,
                inner_rect,
                outer_margin_chunks[2],
            );
        }
        AppState::CustomDurationInput => {
            render_main_menu(frame, app, inner_rect, outer_margin_chunks[2]);
            render_custom_duration_input_popup(frame, app);
        }
        AppState::Quitting => {}
    }
}

fn render_main_menu(
    frame: &mut Frame,
    app: &App,
    area: ratatui::layout::Rect,
    nav_area: ratatui::layout::Rect,
) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(40), Constraint::Min(0)])
        .split(area);

    let left_panel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(0)])
        .split(main_chunks[0]);

    let sessions_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " Select Sessions ",
            Style::default()
                .fg(TITLE_COLOR)
                .add_modifier(Modifier::BOLD),
        ))
        .border_style(
            Style::default().fg(if let PanelFocus::Sessions = app.panel_focus {
                FOCUS_BORDER_COLOR
            } else {
                NORMAL_BORDER_COLOR
            }),
        )
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(SELECTED_ITEM_FG));
    frame.render_widget(sessions_block.clone(), left_panel_chunks[0]);
    let inner_sessions_area = sessions_block.inner(left_panel_chunks[0]);

    let menu_items: Vec<ListItem> = app
        .menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected_menu_index
                && matches!(app.panel_focus, PanelFocus::Sessions)
            {
                Style::default().fg(SELECTED_ITEM_FG).bg(SELECTED_ITEM_BG)
            } else {
                Style::default().fg(ACCENT_COLOR)
            };
            ListItem::new(item.to_string()).style(style)
        })
        .collect();

    let menu_list = List::new(menu_items)
        .block(Block::default())
        .highlight_symbol(if matches!(app.panel_focus, PanelFocus::Sessions) {
            "> "
        } else {
            "  "
        })
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(menu_list, inner_sessions_area);

    let recent_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " Recent ",
            Style::default()
                .fg(TITLE_COLOR)
                .add_modifier(Modifier::BOLD),
        ))
        .border_style(
            Style::default().fg(if let PanelFocus::Recent = app.panel_focus {
                FOCUS_BORDER_COLOR
            } else {
                NORMAL_BORDER_COLOR
            }),
        )
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(SELECTED_ITEM_FG));
    frame.render_widget(recent_block.clone(), left_panel_chunks[1]);
    let inner_recent_area = recent_block.inner(left_panel_chunks[1]);

    let recent_items: Vec<ListItem> = app
        .recent_files
        .iter()
        .enumerate()
        .map(|(i, file_name)| {
            let style = if i == app.selected_recent_index
                && matches!(app.panel_focus, PanelFocus::Recent)
            {
                Style::default().fg(SELECTED_ITEM_FG).bg(SELECTED_ITEM_BG)
            } else {
                Style::default().fg(FILE_ITEM_COLOR)
            };
            ListItem::new(file_name.clone()).style(style)
        })
        .collect();

    let recent_list = List::new(recent_items)
        .block(Block::default())
        .highlight_symbol(if matches!(app.panel_focus, PanelFocus::Recent) {
            "> "
        } else {
            "  "
        })
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(recent_list, inner_recent_area);

    let right_panel_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(NORMAL_BORDER_COLOR))
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(SELECTED_ITEM_FG));
    frame.render_widget(right_panel_block.clone(), main_chunks[1]);
    let inner_right_area = right_panel_block.inner(main_chunks[1]);

    if let Some(content) = &app.displayed_file_content {
        let file_content_paragraph = Paragraph::new(content.as_str())
            .wrap(Wrap { trim: true })
            .scroll((0, 0))
            .style(Style::default().fg(SELECTED_ITEM_FG));
        frame.render_widget(file_content_paragraph, inner_right_area);
    } else {
        let ascii_art = vec![
            "░█▀▀░█▀▄░█▀▀░█▀▀░█░█░█▀▄░▀█▀░▀█▀░█▀▀░",
            "░█▀▀░█▀▄░█▀▀░█▀▀░█▄█░█▀▄░░█░░░█░░█▀▀░",
            " ▀░░░▀░▀░▀▀▀░▀▀▀░▀░▀░▀░▀░▀▀▀░░▀░░▀▀▀░",
        ];
        let mut combined_lines: Vec<Line> = Vec::new();

        combined_lines.push(Line::from(Span::raw("")));

        for s in ascii_art.iter() {
            combined_lines.push(Line::from(vec![
                Span::raw(*s).style(
                    Style::default()
                        .fg(ACCENT_COLOR)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]));
        }

        combined_lines.push(Line::from(Span::raw("")));

        combined_lines.push(Line::from(vec![
            Span::raw("TUI Based tools for free writing without distractions.")
                .style(Style::default().fg(INFO_COLOR)),
        ]));
        combined_lines.push(Line::from(vec![
            Span::raw(format!(
                "{}",
                Local::now().format("%A, %Y-%m-%d %H:%M").to_string()
            ))
            .style(Style::default().fg(INFO_COLOR)),
        ]));
        combined_lines.push(Line::from(Span::raw("")));

        let title_paragraph = Paragraph::new(combined_lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });
        frame.render_widget(title_paragraph, inner_right_area);
    }

    let nav_hint = format!("  [k] up  [j] down  [h] left  [l] right  [Enter] select  [q] quit  ");
    let nav_paragraph = Paragraph::new(nav_hint)
        .alignment(Alignment::Center)
        .style(Style::default().fg(INFO_COLOR));
    frame.render_widget(nav_paragraph, nav_area);
}

fn render_freewrite_session(
    frame: &mut Frame,
    app: &App,
    area: ratatui::layout::Rect,
    nav_area: ratatui::layout::Rect,
) {
    let freewrite_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(area);

    let elapsed_seconds = app.start_time.elapsed().as_secs();
    let time_left_seconds = app
        .session_duration
        .as_secs()
        .saturating_sub(elapsed_seconds);
    let minutes = time_left_seconds / 60;
    let seconds = time_left_seconds % 60;
    let word_count = app.get_word_count();

    let status_line = format!(
        "Press Escape to go back. Time left: {:02}:{:02} | Word Count: {}",
        minutes, seconds, word_count
    );
    let status_paragraph = Paragraph::new(status_line).style(Style::default().fg(SELECTED_ITEM_FG));
    frame.render_widget(status_paragraph, freewrite_chunks[0]);

    let text_area_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " Writing Area ",
            Style::default()
                .fg(TITLE_COLOR)
                .add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default().fg(NORMAL_BORDER_COLOR))
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(SELECTED_ITEM_FG));

    frame.render_widget(text_area_block.clone(), freewrite_chunks[1]);
    let inner_text_area = text_area_block.inner(freewrite_chunks[1]);

    let mut cursor_x: u16 = 0;
    let mut cursor_y: u16 = 0;
    let mut total_virtual_lines: u16 = 0;

    let max_width = inner_text_area.width;

    for char_in_text in app.text.chars() {
        if char_in_text == '\n' {
            total_virtual_lines += 1;
            cursor_y += 1;
            cursor_x = 0;
        } else {
            cursor_x += 1;
            if cursor_x >= max_width {
                total_virtual_lines += 1;
                cursor_y += 1;
                cursor_x = 0;
            }
        }
    }
    if app.text.is_empty() {
        total_virtual_lines = 1;
    } else if cursor_x > 0 {
        total_virtual_lines += 1;
    }

    let scroll_offset_y = if total_virtual_lines > inner_text_area.height {
        total_virtual_lines.saturating_sub(inner_text_area.height)
    } else {
        0
    };

    let text_paragraph = Paragraph::new(app.text.as_str())
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(SELECTED_ITEM_FG))
        .scroll((scroll_offset_y, 0));

    frame.render_widget(text_paragraph, inner_text_area);

    let final_cursor_y = cursor_y.saturating_sub(scroll_offset_y);

    frame.set_cursor_position(Position::new(
        inner_text_area.x + cursor_x,
        inner_text_area.y + final_cursor_y,
    ));

    let nav_hint_freewrite = format!("  [Escape] back to menu  [Ctrl+C] quit  ");
    let nav_paragraph_freewrite = Paragraph::new(nav_hint_freewrite)
        .alignment(Alignment::Center)
        .style(Style::default().fg(INFO_COLOR));
    frame.render_widget(nav_paragraph_freewrite, nav_area);
}

fn render_session_end(
    frame: &mut Frame,
    final_filename: &str,
    word_count: usize,
    wpm: f64,
    area: ratatui::layout::Rect,
    nav_area: ratatui::layout::Rect,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " Session Ended ",
            Style::default()
                .fg(TITLE_COLOR)
                .add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default().fg(NORMAL_BORDER_COLOR))
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(SELECTED_ITEM_FG));

    frame.render_widget(block.clone(), area);
    let inner_summary_area = block.inner(area);

    let summary_lines = vec![
        Line::from(vec![Span::styled(
            "Freewrite Done!",
            Style::default()
                .fg(SUCCESS_COLOR)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::raw(format!("Your writing has been saved to:"))
                .style(Style::default().fg(INFO_COLOR)),
        ]),
        Line::from(vec![Span::styled(
            format!("{}", final_filename),
            Style::default().fg(FOCUS_BORDER_COLOR),
        )]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::raw(format!("Word Count: {}", word_count)).style(Style::default().fg(INFO_COLOR)),
        ]),
        Line::from(vec![
            Span::raw(format!("Typing Speed: {:.2} word/minutes", wpm))
                .style(Style::default().fg(INFO_COLOR)),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::styled(
            "Press [Enter] to go back to main menu...",
            Style::default()
                .fg(ACCENT_COLOR)
                .add_modifier(Modifier::BOLD),
        )]),
    ];

    let paragraph = Paragraph::new(summary_lines)
        .alignment(Alignment::Center)
        .style(Style::default().fg(SELECTED_ITEM_FG));
    frame.render_widget(paragraph, inner_summary_area);

    let nav_hint_end = format!("  [Enter] back to menu  [q] quit  ");
    let nav_paragraph_end = Paragraph::new(nav_hint_end)
        .alignment(Alignment::Center)
        .style(Style::default().fg(INFO_COLOR));
    frame.render_widget(nav_paragraph_end, nav_area);
}

fn render_custom_duration_input_popup(frame: &mut Frame, app: &App) {
    let size = frame.area();

    // Create centered popup
    let popup_width = 55;
    let popup_height = 11;
    let popup_x = (size.width.saturating_sub(popup_width)) / 2;
    let popup_y = (size.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

    // Clear and render popup container
    frame.render_widget(Clear, popup_area);

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " Custom Duration (minutes) ",
            Style::default()
                .fg(TITLE_COLOR)
                .add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default().fg(FOCUS_BORDER_COLOR))
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(SELECTED_ITEM_FG));

    let inner_area = popup_block.inner(popup_area);
    frame.render_widget(popup_block, popup_area);

    // Create main layout sections
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Top spacing + prompt
            Constraint::Length(3), // Input field with borders
            Constraint::Length(1), // Middle spacing
            Constraint::Length(1), // Hint
            Constraint::Length(1), // Bottom padding
        ])
        .split(inner_area);

    // Render prompt section
    let prompt_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Top padding
            Constraint::Length(1), // Actual prompt
        ])
        .split(sections[0]);

    let prompt = Paragraph::new(format!(
        "Enter duration (default {}):",
        DEFAULT_SESSION_DURATION_MINS
    ))
    .style(Style::default().fg(INFO_COLOR))
    .alignment(Alignment::Center);
    frame.render_widget(prompt, prompt_layout[1]);

    // Render input section
    let input_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(5), // Left padding
            Constraint::Min(20),   // Input field
            Constraint::Length(5), // Right padding
        ])
        .split(sections[1]);

    let input_field = Paragraph::new(app.custom_duration_input_text.as_str())
        .style(
            Style::default()
                .fg(SELECTED_ITEM_FG)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(NORMAL_BORDER_COLOR))
                .border_type(ratatui::widgets::BorderType::Plain),
        );
    frame.render_widget(input_field, input_layout[1]);

    // Render hint section
    let hint_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Actual hint
            Constraint::Length(1), // Bottom Padding
        ])
        .split(sections[3]);

    let hint = Paragraph::new("[Enter] Confirm [Esc] Cancel")
        .style(
            Style::default()
                .fg(INFO_COLOR)
                .add_modifier(Modifier::ITALIC),
        )
        .alignment(Alignment::Center);
    frame.render_widget(hint, hint_layout[0]);

    // Set cursor position
    let cursor_x = input_layout[1].x + 1 + app.custom_duration_input_text.len() as u16;
    let cursor_y = input_layout[1].y + 1;
    frame.set_cursor_position(Position::new(cursor_x, cursor_y));
}
