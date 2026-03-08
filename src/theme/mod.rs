pub mod builtin;

use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Theme {
    pub name: String,
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub accent_dim: Color,
    pub gutter_bg: Color,
    pub gutter_fg: Color,
    pub active_line_bg: Color,
    pub selection_bg: Color,
    pub status_bg: Color,
    pub status_fg: Color,
    pub tab_bg: Color,
    pub tab_active_bg: Color,
    pub tab_fg: Color,
    pub tab_active_fg: Color,
    pub sidebar_bg: Color,
    pub sidebar_fg: Color,
    pub sidebar_active_bg: Color,
    pub border: Color,
    pub keyword: Color,
    pub string: Color,
    pub comment: Color,
    pub function: Color,
    pub r#type: Color,
    pub number: Color,
    pub operator: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
    pub popup_bg: Color,
    pub popup_border: Color,
}

impl Theme {
    pub fn all() -> Vec<Theme> {
        let mut themes = vec![
            builtin::aether_dark(),
            builtin::aether_light(),
            builtin::ember(),
            builtin::frost(),
            builtin::midnight(),
            builtin::sakura(),
            builtin::void(),
            builtin::sea_software_dark(),
            builtin::sea_software_light(),
            builtin::solarized(),
            builtin::dracula(),
        ];

        // Load custom themes from config dir
        if let Some(custom) = load_custom_themes() {
            themes.extend(custom);
        }

        themes
    }

    pub fn names() -> Vec<String> {
        Self::all().iter().map(|t| t.name.clone()).collect()
    }

    #[allow(dead_code)]
    pub fn builtin_count() -> usize {
        11
    }
}

/// Custom theme definition in TOML
#[derive(Debug, Deserialize)]
struct CustomThemeDef {
    name: String,
    bg: String,
    fg: String,
    accent: String,
    accent_dim: Option<String>,
    gutter_fg: Option<String>,
    border: Option<String>,
    keyword: Option<String>,
    string: Option<String>,
    comment: Option<String>,
    function: Option<String>,
    r#type: Option<String>,
    number: Option<String>,
    error: Option<String>,
}

fn parse_hex_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 { return None; }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}

fn darken(color: Color, amount: u8) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            r.saturating_sub(amount),
            g.saturating_sub(amount),
            b.saturating_sub(amount),
        ),
        c => c,
    }
}

fn lighten(color: Color, amount: u8) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            r.saturating_add(amount),
            g.saturating_add(amount),
            b.saturating_add(amount),
        ),
        c => c,
    }
}

fn load_custom_themes() -> Option<Vec<Theme>> {
    let themes_dir = dirs::config_dir()?.join("aether").join("themes");
    if !themes_dir.exists() { return None; }

    let mut themes = Vec::new();
    let Ok(entries) = std::fs::read_dir(&themes_dir) else { return None };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("toml") {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(def) = toml::from_str::<CustomThemeDef>(&content) {
                    if let Some(theme) = build_custom_theme(def) {
                        themes.push(theme);
                    }
                }
            }
        }
    }

    if themes.is_empty() { None } else { Some(themes) }
}

fn build_custom_theme(def: CustomThemeDef) -> Option<Theme> {
    let bg = parse_hex_color(&def.bg)?;
    let fg = parse_hex_color(&def.fg)?;
    let accent = parse_hex_color(&def.accent)?;
    let accent_dim = def.accent_dim.as_deref().and_then(parse_hex_color).unwrap_or(darken(accent, 40));
    let border = def.border.as_deref().and_then(parse_hex_color).unwrap_or(lighten(bg, 30));
    let gutter_fg = def.gutter_fg.as_deref().and_then(parse_hex_color).unwrap_or(darken(fg, 60));
    let keyword = def.keyword.as_deref().and_then(parse_hex_color).unwrap_or(accent);
    let string = def.string.as_deref().and_then(parse_hex_color).unwrap_or(lighten(accent, 40));
    let comment = def.comment.as_deref().and_then(parse_hex_color).unwrap_or(gutter_fg);
    let function = def.function.as_deref().and_then(parse_hex_color).unwrap_or(lighten(accent, 20));
    let type_color = def.r#type.as_deref().and_then(parse_hex_color).unwrap_or(accent_dim);
    let number = def.number.as_deref().and_then(parse_hex_color).unwrap_or(lighten(accent, 30));
    let error = def.error.as_deref().and_then(parse_hex_color).unwrap_or(Color::Rgb(255, 80, 80));

    Some(Theme {
        name: def.name,
        bg,
        fg,
        accent,
        accent_dim,
        gutter_bg: darken(bg, 5),
        gutter_fg,
        active_line_bg: lighten(bg, 10),
        selection_bg: lighten(bg, 20),
        status_bg: lighten(bg, 8),
        status_fg: fg,
        tab_bg: darken(bg, 5),
        tab_active_bg: bg,
        tab_fg: gutter_fg,
        tab_active_fg: fg,
        sidebar_bg: darken(bg, 5),
        sidebar_fg: darken(fg, 20),
        sidebar_active_bg: lighten(bg, 10),
        border,
        keyword,
        string,
        comment,
        function,
        r#type: type_color,
        number,
        operator: accent_dim,
        error,
        warning: Color::Rgb(224, 175, 104),
        success: string,
        popup_bg: lighten(bg, 5),
        popup_border: accent,
    })
}
