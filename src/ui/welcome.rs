use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

const LOGO: &str = r#"
    _       _   _               
   / \   ___| |_| |__   ___ _ __ 
  / _ \ / _ \ __| '_ \ / _ \ '__|
 / ___ \  __/ |_| | | |  __/ |   
/_/   \_\___|\__|_| |_|\___|_|   
"#;

const TAGLINE: &str = "[ a   t e x t   e d i t o r ]";

pub fn draw_welcome(frame: &mut Frame, app: &mut App) {
    let theme = &app.theme;
    let area = frame.area();

    // Fill background
    frame.render_widget(
        Block::default().style(Style::default().bg(theme.bg)),
        area,
    );

    let _center_x = area.width / 2;
    let center_y = area.height / 2;

    let mut lines: Vec<Line> = Vec::new();

    // Top padding
    let logo_height = 8;
    let content_height = logo_height + 15; // logo + spacing + options + footer
    let start_y = center_y.saturating_sub(content_height / 2);

    for _ in 0..start_y {
        lines.push(Line::from(""));
    }

    // Logo - color each line with gradient
    let logo_colors = [
        theme.accent,      // bright
        theme.accent,
        theme.accent,
        theme.accent_dim,
        theme.accent_dim,
        theme.border,
        theme.border,
    ];

    for (i, line) in LOGO.lines().skip(1).enumerate() {
        let color = logo_colors.get(i).copied().unwrap_or(theme.accent);
        let padded = format!("{:^width$}", line, width = area.width as usize);
        lines.push(Line::from(Span::styled(padded, Style::default().fg(color).add_modifier(Modifier::BOLD))));
    }

    // Tagline
    lines.push(Line::from(""));
    let tagline = format!("{:^width$}", TAGLINE, width = area.width as usize);
    lines.push(Line::from(Span::styled(tagline, Style::default().fg(theme.accent_dim))));

    // Version
    let ver_str = include_str!("../../version.txt").trim();
    let version = format!("{:^width$}", format!("v{}", ver_str), width = area.width as usize);
    lines.push(Line::from(Span::styled(version, Style::default().fg(theme.comment))));

    lines.push(Line::from(""));
    lines.push(Line::from(""));

    // Welcome message
    let greeting = format!("Welcome back, {}!", app.config.username);
    let greeting_line = format!("{:^width$}", greeting, width = area.width as usize);
    lines.push(Line::from(Span::styled(greeting_line, Style::default().fg(theme.fg).add_modifier(Modifier::BOLD))));

    lines.push(Line::from(""));

    // Menu options
    let mut options = vec![
        ("n", "New File", "Create a new empty file"),
        ("o", "Open File", "Browse and open a file"),
        ("a", "About", "Credits & Built by Wyind"),
        ("s", "Settings", "Configure Aether"),
    ];
    if app.config.auto_update {
        options.push(("u", "Update", "Check for and install updates"));
    }
    options.push(("q", "Quit", "Exit Aether"));

    for (i, (key, label, desc)) in options.iter().enumerate() {
        let is_selected = i == app.welcome_state.selected_option;

        let line_text = if is_selected {
            format!("{:^width$}", format!("▸  [{}]  {}  —  {}", key, label, desc), width = area.width as usize)
        } else {
            format!("{:^width$}", format!("   [{}]  {}  —  {}", key, label, desc), width = area.width as usize)
        };

        let style = if is_selected {
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };

        lines.push(Line::from(Span::styled(line_text, style)));
    }

    // Recent files
    if !app.welcome_state.recent_files.is_empty() {
        lines.push(Line::from(""));
        let header = format!("{:^width$}", "── Recent Files ──", width = area.width as usize);
        lines.push(Line::from(Span::styled(header, Style::default().fg(theme.accent_dim))));
        lines.push(Line::from(""));

        for (i, file) in app.welcome_state.recent_files.iter().take(5).enumerate() {
            let opt_idx = i + options.len();
            let is_selected = opt_idx == app.welcome_state.selected_option;
            // Show just filename, not full path
            let display_name = std::path::Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(file);

            let line_text = if is_selected {
                format!("{:^width$}", format!("▸  {}  ({})", display_name, file), width = area.width as usize)
            } else {
                format!("{:^width$}", format!("   {}  ({})", display_name, file), width = area.width as usize)
            };

            let style = if is_selected {
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.comment)
            };

            lines.push(Line::from(Span::styled(line_text, style)));
        }
    }

    // Footer
    lines.push(Line::from(""));
    lines.push(Line::from(""));
    let footer = format!("{:^width$}", format!("Theme: {} │ Mode: {} │ Ctrl+P: Command Palette",
        theme.name,
        match &app.edit_mode {
            crate::app::EditMode::Vim => "Vim",
            crate::app::EditMode::Nano => "Nano",
            crate::app::EditMode::Emacs => "Emacs",
            crate::app::EditMode::Aether => "Aether",
        },
    ), width = area.width as usize);
    lines.push(Line::from(Span::styled(footer, Style::default().fg(theme.accent_dim))));

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}
