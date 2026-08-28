use crossterm::execute;
use crossterm::terminal::{size, EnterAlternateScreen, LeaveAlternateScreen};
use std::env;
use std::fs;
use std::io::stdout;
use std::path::Path;

#[path = "../lib/ansi_codes.rs"]
mod ansi;

#[path = "../lib/keyboard.rs"]
mod keyboard;

#[path = "../lang/rust.rs"]
mod rust;

const ARROWS: [&str; 4] = ["up", "down", "left", "right"];
const COLS: usize = 50;
const LINES: usize = 1000;
const VISIBLE_LINES: usize = 21;

pub fn clear_screen_ansi() {
    print!("\x1B[3J\x1B[2J\x1B[1;1H");

    use std::io::Write;
    let _ = std::io::stdout().flush();
}

fn save_file(code: &[[char; COLS]; LINES], filename: &str) -> std::io::Result<()> {
    let mut contents = String::new();

    for row in 0..LINES {
        let line: String = code[row].iter().collect();
        contents.push_str(line.trim_end());
        contents.push('\n');
    }

    std::fs::write(filename, contents)
}

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    let _ = execute!(stdout(), EnterAlternateScreen);

    let mut code: [[char; COLS]; LINES] = [[' '; COLS]; LINES];

    // Read file argument passed via CLI: `tidy [filename]`
    let args: Vec<String> = env::args().collect();
    let mut current_filename: Option<String> = None;

    if let Some(path_arg) = args.get(1) {
        current_filename = Some(path_arg.clone());
        if Path::new(path_arg).exists() {
            if let Ok(content) = fs::read_to_string(path_arg) {
                for (r, line_str) in content.lines().enumerate().take(LINES) {
                    for (c, ch) in line_str.chars().enumerate().take(COLS) {
                        code[r][c] = ch;
                    }
                }
            }
        }
    }

    let mut is_caps = false;

    let mut line: usize = 0;
    let mut col: usize = 0;

    let mut needs_render = true;

    let mut save_mode = false;
    let mut filename = current_filename.clone().unwrap_or_default();
    let mut status_message = String::new();

    loop {
        if let Some(key) = keyboard::get_pressed_key_string() {
            needs_render = true;

            if save_mode {
                if key == "esc" {
                    save_mode = false;
                    filename = current_filename.clone().unwrap_or_default();
                    status_message.clear();
                } else if key == "backspace" {
                    filename.pop();
                } else if key == "enter" {
                    if !filename.is_empty() {
                        match save_file(&code, &filename) {
                            Ok(_) => {
                                status_message = format!("Saved {}", filename);
                                current_filename = Some(filename.clone());
                                save_mode = false;
                            }

                            Err(error) => {
                                status_message = format!("Error saving: {}", error);
                                save_mode = false;
                            }
                        }
                    }
                } else if key == "space" {
                    filename.push(' ');
                } else if key.chars().count() == 1 {
                    filename.push_str(&key);
                }

                continue;
            }

            if key == "esc" {
                break;
            }

            if key == "ctrl+s" {
                save_mode = true;
                filename = current_filename.clone().unwrap_or_default();
                status_message.clear();
                continue;
            }

            if key == "caps" {
                is_caps = !is_caps;
            }

            if key == "space" {
                if col < COLS - 1 {
                    code[line][col] = ' ';
                    col += 1;
                }
            }

            if key == "backspace" {
                if col > 0 {
                    col -= 1;
                    code[line][col] = ' ';
                }
            }

            if key == "enter" {
                if line < LINES - 1 {
                    line += 1;
                    col = 0;
                }
            }

            if ARROWS.contains(&key.as_str()) {
                match key.as_str() {
                    "up" => {
                        if line > 0 {
                            line -= 1;
                        }
                    }

                    "down" => {
                        if line < LINES - 1 {
                            line += 1;
                        }
                    }

                    "left" => {
                        if col > 0 {
                            col -= 1;
                        }
                    }

                    "right" => {
                        if col < COLS - 1 {
                            col += 1;
                        }
                    }

                    _ => {}
                }
            }

            if key.chars().count() == 1 {
                let mut ch = key.chars().next().unwrap();

                if ch.is_alphabetic() {
                    if is_caps {
                        ch = ch.to_ascii_uppercase();
                    } else {
                        ch = ch.to_ascii_lowercase();
                    }
                }

                if !ch.is_control() && col < COLS && line < LINES {
                    code[line][col] = ch;

                    if col < COLS - 1 {
                        col += 1;
                    }
                }
            }
        }

        if needs_render {
            clear_screen_ansi();

            for row in 0..VISIBLE_LINES {
                let cursor = if row == line {
                    Some(col)
                } else {
                    None
                };

                let line_str = ansi::highlight_line(
                    &code[row],
                    &rust::HIGHLIGHTS,
                    cursor,
                );

                println!("{}\r", line_str);
            }

            // Get terminal width to draw status bar edge-to-edge full width
            let (term_cols, _) = size().unwrap_or((80, 24));
            let term_width = term_cols as usize;

            let file_label = match &current_filename {
                Some(name) => format!("[{}]", name),
                None => "[New File]".to_string(),
            };

            if save_mode {
                let left_text = format!(" Save file: {}", filename);
                let padding = term_width.saturating_sub(left_text.len());
                print!(
                    "\x1B[7m{}{}\x1B[0m\r\n",
                    left_text,
                    " ".repeat(padding)
                );
            } else {
                let left_text = "^S Save    Esc Exit".to_string();
                let total_text_len = left_text.len() + file_label.len();
                
                if term_width > total_text_len {
                    let middle_padding = term_width - total_text_len;
                    print!(
                        "\x1B[7m{}{}{}\x1B[0m\r\n",
                        left_text,
                        " ".repeat(middle_padding),
                        file_label
                    );
                } else {
                    print!("\x1B[7m{}\x1B[0m\r\n", left_text);
                }
            }

            if !status_message.is_empty() {
                println!("{}", status_message);
            } else {
                println!();
            }

            needs_render = false;

            use std::io::Write;
            let _ = stdout().flush();
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    let _ = execute!(stdout(), LeaveAlternateScreen);
    crossterm::terminal::disable_raw_mode().unwrap();
}