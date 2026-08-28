#![allow(dead_code)]

use crate::ansi;

pub const TYPES: [&str; 16] = [
    "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128",
    "usize", "isize", "String", "bool", "f32", "f64",
];

pub const KEYWORDS: [&str; 28] = [
    "as", "async", "const", "crate", "dyn", "enum", "extern", "false", "fn",
    "for", "impl", "let", "mod", "move", "mut", "pub", "ref", "self", "Self",
    "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where",
];

pub const HIGHLIGHTS: [(&str, &str); 44] = [
    ("i8", ansi::GREEN),
    ("i16", ansi::GREEN),
    ("i32", ansi::GREEN),
    ("i64", ansi::GREEN),
    ("i128", ansi::GREEN),
    ("u8", ansi::GREEN),
    ("u16", ansi::GREEN),
    ("u32", ansi::GREEN),
    ("u64", ansi::GREEN),
    ("u128", ansi::GREEN),
    ("usize", ansi::GREEN),
    ("isize", ansi::GREEN),
    ("String", ansi::GREEN),
    ("bool", ansi::GREEN),
    ("f32", ansi::GREEN),
    ("f64", ansi::GREEN),

    ("as", ansi::CYAN),
    ("async", ansi::CYAN),
    ("const", ansi::CYAN),
    ("crate", ansi::CYAN),
    ("dyn", ansi::CYAN),
    ("enum", ansi::CYAN),
    ("extern", ansi::CYAN),
    ("false", ansi::CYAN),
    ("fn", ansi::CYAN),
    ("for", ansi::CYAN),
    ("impl", ansi::CYAN),
    ("let", ansi::CYAN),
    ("mod", ansi::CYAN),
    ("move", ansi::CYAN),
    ("mut", ansi::CYAN),
    ("pub", ansi::CYAN),
    ("ref", ansi::CYAN),
    ("self", ansi::CYAN),
    ("Self", ansi::CYAN),
    ("static", ansi::CYAN),
    ("struct", ansi::CYAN),
    ("super", ansi::CYAN),
    ("trait", ansi::CYAN),
    ("true", ansi::CYAN),
    ("type", ansi::CYAN),
    ("unsafe", ansi::CYAN),
    ("use", ansi::CYAN),
    ("where", ansi::CYAN),
];

pub fn highlight_line<const N: usize>(
    line: &[char; N],
    cursor: Option<usize>,
) -> String {
    ansi::highlight_line_generic(line, cursor, |line_buf, start| {
        let mut i = start;
        while i < N && (line_buf[i].is_alphanumeric() || line_buf[i] == '_') {
            i += 1;
        }

        if i < N && line_buf[i] == '!' {
            i += 1;
            return (i, Some(ansi::B_RED));
        }

        let word: String = line_buf[start..i].iter().collect();
        for &(highlight_word, highlight_color) in &HIGHLIGHTS {
            if word == highlight_word {
                return (i, Some(highlight_color));
            }
        }

        (i, None)
    })
}