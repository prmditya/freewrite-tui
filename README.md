# `freewrite-tui`

A simple and elegant Terminal User Interface (TUI) application designed for distraction-free freewriting sessions. Write freely without the clutter of traditional word processors, focusing purely on your thoughts.

## About

`freewrite-tui` is built with Rust and the `ratatui` library, offering a clean, minimalist environment for writers. It encourages continuous writing by providing timed sessions and automatic saving, helping you overcome writer's block and capture your ideas efficiently.

## Features

- **Distraction-Free Environment:** A clean terminal interface to keep you focused.
- **Customizable Session Durations:** Choose from predefined 5, 10, 20-minute sessions, or set a custom duration via an interactive TUI popup.
- **Automatic Session Saving:** Your writing sessions are automatically saved to timestamped `.txt` files in a dedicated directory.
- **Session Summary:** Get a summary including word count and Words Per Minute (WPM) at the end of each session.
- **Recent Files Browser:** Easily view your past freewrite sessions directly from the main menu.
- **Intuitive Navigation:** Simple keyboard controls for navigating menus and writing.
- **Minimalist Design:** Clean aesthetics.

## Installation

Before you begin, ensure you have [Rust and Cargo](https://rustup.rs/) installed on your system.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/prmditya/freewrite-tui.git
    cd freewrite-tui
    ```
2.  **Run the application:**
    ```bash
    cargo run --release
    ```
    The `--release` flag compiles the application with optimizations, making it run faster.
3.  **Or you can build the application:**
    ```bash
    cargo build --release
    ```
    The `--release` flag compiles the application with optimizations, making it run faster.

## Usage

Once you run `cargo run --release`, the `freewrite-tui` application will launch in your terminal.

If you build the application you can launch it anywhere **but for now it'll save the notes relative with directory that you currently run the application!!**.

### Main Menu Navigation

- Use `j` / `k` (or Arrow Down / Arrow Up) to navigate up and down the "Select Sessions" and "Recent" lists.
- Use `h` / `l` (or Arrow Left / Arrow Right) to switch focus between the "Select Sessions" panel and the "Recent" panel.
- Press `Enter` to select a highlighted option (start a session or view a recent file).
- Press `q` to quit the application from the main menu.

### Freewriting Session

- Simply start typing\! Your text will appear in the main writing area.
- The status bar at the top will show the remaining time and your current word count.
- **To end a session early and save:** Press `Esc`. This will take you to the Session End Summary.

### Custom Duration Input

- When you select "Custom duration" from the main menu, a popup will appear.
- Type the desired duration in minutes (e.g., `15` for 15 minutes).
- Press `Enter` to confirm and start the session.
- Press `Esc` to cancel and return to the main menu.

### Session End Summary

- After your session ends (either by time expiring or pressing `Esc`), a summary screen will appear showing:
  - The filename where your writing was saved.
  - Total word count for the session.
  - Your Words Per Minute (WPM) for the session.
- Press `Enter` to return to the main menu.
- Press `q` to quit the application.

## Configuration

`freewrite-tui` saves your sessions in a directory named `freewrite_sessions` in the directory where you run the application.

You can customize various application settings, including default session duration and UI colors, by modifying the `src/config.rs` file in the source code.

## Screenshots

![image](https://github.com/user-attachments/assets/9de8029f-a894-4c2b-830c-9fc6a636948b)
![image](https://github.com/user-attachments/assets/7405502b-fad7-41dc-85df-37d853ba96c0)
![image](https://github.com/user-attachments/assets/390ab791-a47c-41a6-af11-730192b6a130)
![image](https://github.com/user-attachments/assets/852ebb7c-e1dd-4442-9ff6-b70ba8ed7ccc)

## Contributing

Contributions are welcome\! If you have suggestions for improvements, bug reports, or would like to contribute code, please feel free to:

1.  Open an [issue](https://www.google.com/search?q=https://github.com/prmditya/freewrite-tui/issues) to discuss changes.
2.  Fork the repository and submit a [pull request](https://www.google.com/search?q=https://github.com/prmditya/freewrite-tui/pulls).

## License

This project is licensed under the **MIT License**.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Terminal UI powered by [ratatui](https://ratatui.rs/)
- Event handling via [crossterm](https://docs.rs/crossterm/latest/crossterm/)
- Date and time functionalities by [chrono](https://docs.rs/chrono/latest/chrono/)

---
