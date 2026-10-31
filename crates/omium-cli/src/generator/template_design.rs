//! Shared dark welcome design for Standard, Vue, and React scaffolds.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

const SHARED_CSS: &str = include_str!("../../templates/shared/design/app.css");

/// Askama layout matching Vue/React `AppLayout` structure.
#[must_use]
pub fn standard_layout_html() -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="color-scheme" content="dark">
  <title>{{% block title %}}Page{{% endblock %}} — {{{{ app_name }}}}</title>
  <style>
{css}
  </style>
</head>
<body class="antialiased">
  <div class="app-shell">
    <header class="app-header">
      <div class="app-header-inner">
        <div>
          <p class="brand">{{{{ app_name }}}}</p>
          <h1 class="page-title">{{% block header_title %}}Welcome{{% endblock %}}</h1>
          <p class="page-subtitle">{{% block header_subtitle %}}{{% endblock %}}</p>
        </div>
        <nav class="app-nav" aria-label="Main">
          <a class="nav-link nav-link-active" href="/">Home</a>
          <a class="nav-link" href="/health">Health</a>
        </nav>
      </div>
    </header>
    <main class="app-main">
      {{% block content %}}{{% endblock %}}
    </main>
  </div>
</body>
</html>
"#,
        css = SHARED_CSS
    )
}

/// Welcome page body (extends [`standard_layout_html`]).
#[must_use]
pub fn standard_welcome_html() -> &'static str {
    r#"{% extends "layout.html" %}

{% block title %}Welcome{% endblock %}
{% block header_title %}Welcome{% endblock %}
{% block header_subtitle %}Server-rendered HTML with the same dark Omium starter look as Vue and React apps.{% endblock %}

{% block content %}
<section class="welcome-hero">
  <div class="welcome-glow" aria-hidden="true"></div>
  <p class="hero-eyebrow">You're all set</p>
  <h2 class="hero-title">{{ greeting }}</h2>
  <p class="hero-lead">{{ hint }}</p>
  <div class="pill-row">
    <span class="pill pill-accent">{{ mode_label }}</span>
    <span class="pill">Axum</span>
    <span class="pill">Askama</span>
    <span class="pill">Omium</span>
  </div>
  <div class="feature-grid">
    <div class="feature-tile">
      <strong>Typed views</strong>
      <span>Compile-time checked templates and context structs.</span>
    </div>
    <div class="feature-tile">
      <strong>No Node required</strong>
      <span>Ship HTML from Rust without a frontend build step.</span>
    </div>
    <div class="feature-tile">
      <strong>Same visual language</strong>
      <span>Swap to Vue or React later without redesigning the shell.</span>
    </div>
  </div>
</section>
{% endblock %}
"#
}

/// Writes `resources/css/app.css` (Vue/React) from the shared design file.
pub fn write_vite_app_css(app_root: &Path) -> Result<()> {
    let target = app_root.join("resources/css/app.css");
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&target, SHARED_CSS)
        .with_context(|| format!("write shared CSS to {}", target.display()))?;
    Ok(())
}
