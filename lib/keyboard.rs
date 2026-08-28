use crossterm::event::{self, Event, KeyCode, KeyEventKind, ModifierKeyCode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

#[cfg(target_os = "windows")]
unsafe extern "system" {
    fn GetKeyState(nVirtKey: i32) -> i16;
}

static WAS_CAPS_DOWN: AtomicBool = AtomicBool::new(false);

fn check_windows_caps_press() -> bool {
    #[cfg(target_os = "windows")]
    unsafe {
        let is_down = (GetKeyState(0x14) as u16 & 0x8000) != 0;
        let was_down = WAS_CAPS_DOWN.swap(is_down, Ordering::Relaxed);
        return is_down && !was_down;
    }

    #[cfg(not(target_os = "windows"))]
    false
}

#[allow(dead_code)]
pub fn get_pressed_key_string() -> Option<String> {
    if check_windows_caps_press() {
        return Some("caps".to_string());
    }

    if event::poll(Duration::from_secs(0)).unwrap_or(false) {
        if let Ok(Event::Key(key_event)) = event::read() {
            if key_event.kind != KeyEventKind::Press {
                return None;
            }

            if key_event.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                if let KeyCode::Char(c) = key_event.code {
                    return Some(format!("ctrl+{}", c.to_ascii_lowercase()));
                }
            }

            let name = match key_event.code {
                KeyCode::Char(' ') => "space".to_string(),

                KeyCode::Char(c) => c.to_string(),

                KeyCode::CapsLock => "caps".to_string(),

                KeyCode::Modifier(m) => match m {
                    ModifierKeyCode::LeftShift | ModifierKeyCode::RightShift => {
                        "shift".to_string()
                    }

                    ModifierKeyCode::LeftControl | ModifierKeyCode::RightControl => {
                        "ctrl".to_string()
                    }

                    ModifierKeyCode::LeftAlt | ModifierKeyCode::RightAlt => {
                        "alt".to_string()
                    }

                    _ => format!("{:?}", m).to_lowercase(),
                },

                KeyCode::Esc => "esc".to_string(),
                KeyCode::Tab => "tab".to_string(),
                KeyCode::Enter => "enter".to_string(),
                KeyCode::Backspace => "backspace".to_string(),
                KeyCode::Delete => "del".to_string(),

                KeyCode::Up => "up".to_string(),
                KeyCode::Down => "down".to_string(),
                KeyCode::Left => "left".to_string(),
                KeyCode::Right => "right".to_string(),

                KeyCode::F(num) => format!("f{}", num),

                _ => return None,
            };

            return Some(name);
        }
    }

    None
}