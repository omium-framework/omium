//! Cargo / Rust identifiers from project directory names (Unicode-safe slugs).

use std::path::Path;

/// `[package] name` slug (kebab-case, ASCII) derived from the destination folder name.
#[must_use]
pub fn package_name_from_path(dest: &Path) -> String {
    let raw = dest
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("omium-app");
    slugify_cargo_package_name(raw)
}

/// Rust crate identifier for `[lib] name` and `[dependencies]` keys.
#[must_use]
pub fn lib_name_from_package_name(package_name: &str) -> String {
    let ident = package_name.replace('-', "_");
    if ident.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        format!("app_{ident}")
    } else {
        ident
    }
}

fn slugify_cargo_package_name(input: &str) -> String {
    let mut out = String::new();
    let mut pending_hyphen = false;

    for ch in input.chars() {
        if ch.is_ascii_whitespace() || ch == '-' || ch == '_' {
            if !out.is_empty() {
                pending_hyphen = true;
            }
            continue;
        }

        let mapped = map_char_to_ascii(ch);
        if mapped.is_ascii_alphanumeric() {
            if pending_hyphen {
                out.push('-');
                pending_hyphen = false;
            }
            out.push(mapped.to_ascii_lowercase());
        } else if mapped != '\0' {
            pending_hyphen = !out.is_empty();
        }
    }

    while out.ends_with('-') {
        out.pop();
    }

    if out.is_empty() {
        return String::from("omium-app");
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        format!("app-{out}")
    } else {
        out
    }
}

fn map_char_to_ascii(ch: char) -> char {
    match ch {
        'ı' => 'i',
        'İ' => 'i',
        'ş' | 'Ş' => 's',
        'ğ' | 'Ğ' => 'g',
        'ü' | 'Ü' => 'u',
        'ö' | 'Ö' => 'o',
        'ç' | 'Ç' => 'c',
        '\u{0308}' | '\u{0307}' | '\u{0304}' | '\u{0323}' => '\0',
        c if c.is_ascii_alphanumeric() => c,
        c if c.is_alphabetic() => {
            let folded = c.to_ascii_lowercase();
            if folded.is_ascii_alphanumeric() {
                folded
            } else {
                '\0'
            }
        }
        _ => '\0',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turkish_folder_name_becomes_ascii_slug() {
        assert_eq!(
            slugify_cargo_package_name("adsız klasör"),
            "adsiz-klasor"
        );
        assert_eq!(
            lib_name_from_package_name("adsiz-klasor"),
            "adsiz_klasor"
        );
    }

    #[test]
    fn spaces_and_punctuation_become_hyphens() {
        assert_eq!(slugify_cargo_package_name("My Cool App!!!"), "my-cool-app");
    }

    #[test]
    fn leading_digit_gets_app_prefix() {
        assert_eq!(slugify_cargo_package_name("42demo"), "app-42demo");
        assert_eq!(lib_name_from_package_name("app-42demo"), "app_42demo");
    }

    #[test]
    fn empty_after_sanitize_falls_back() {
        assert_eq!(slugify_cargo_package_name("!!!"), "omium-app");
    }
}
