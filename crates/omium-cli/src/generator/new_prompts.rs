//! Interactive prompts for `omnisan new` when flags are omitted.

use std::io::{self, IsTerminal, Write};

use anyhow::{Context, Result, bail};

use super::new::{FrontendMode, default_app_name_from_path};
use std::path::Path;

/// Reads application name from stdin or derives it from the project path.
pub fn resolve_app_name(path: &Path, flag: Option<&str>, no_interaction: bool) -> Result<String> {
    if let Some(name) = flag {
        return Ok(name.to_string());
    }
    let suggested = default_app_name_from_path(path);
    if no_interaction || !stdin_is_tty() {
        return Ok(suggested);
    }
    prompt_line(
        &format!("Application name [{suggested}]: "),
        Some(suggested.as_str()),
    )
}

/// Reads frontend mode from stdin or uses the default.
pub fn resolve_frontend(flag: Option<FrontendMode>, no_interaction: bool) -> Result<FrontendMode> {
    if let Some(mode) = flag {
        return Ok(mode);
    }
    if no_interaction || !stdin_is_tty() {
        return Ok(FrontendMode::Vue);
    }

    eprintln!("Frontend mode:");
    eprintln!("  1) standard — Askama (server-rendered HTML)");
    eprintln!("  2) vue — Vue + Inertia (recommended)");
    eprintln!("  3) api — API only");
    eprintln!("  4) react — React + Inertia");
    let choice = prompt_line("Choice [2]: ", Some("2"))?;
    match choice.trim() {
        "1" | "standard" => Ok(FrontendMode::Standard),
        "2" | "vue" | "" => Ok(FrontendMode::Vue),
        "3" | "api" => Ok(FrontendMode::Api),
        "4" | "react" => Ok(FrontendMode::React),
        other => bail!("invalid choice: {other}"),
    }
}

fn stdin_is_tty() -> bool {
    io::stdin().is_terminal()
}

fn prompt_line(label: &str, default: Option<&str>) -> Result<String> {
    print!("{label}");
    io::stdout().flush().context("failed to flush stdout")?;
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .context("failed to read stdin")?;
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(default.unwrap_or("").to_string());
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_name_from_path_is_english_title_case() {
        let name = default_app_name_from_path(Path::new("examples/inertia-vue-app"));
        assert_eq!(name, "Inertia Vue App");
    }
}
