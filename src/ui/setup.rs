use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;
use crate::theme::Theme;

pub fn draw_setup(frame: &mut Frame, app: &mut App) {
    let theme_list = Theme::all();
    let preview_theme = &theme_list[app.setup_state.selected_theme];
    let area = frame.area();

    // Use the preview theme for setup to show live preview
    let bg = preview_theme.bg;
    let fg = preview_theme.fg;
    let accent = preview_theme.accent;
    let accent_dim = preview_theme.accent_dim;
    let border_color = preview_theme.border;
    let comment = preview_theme.comment;

    // Fill background
    frame.render_widget(
        Block::default().style(Style::default().bg(bg)),
        area,
    );

    let _center_x = area.width / 2;
    let step = app.setup_state.step;

    // Popup area
    let popup_width = 64.min(area.width.saturating_sub(4));
    let popup_height = 22.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    let block = Block::default()
        .title(Span::styled(
            " 󰄚 Aether Setup ",
            Style::default().fg(accent).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(bg));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    // Progress indicator
    let _progress_text = format!("  Step {} of {}  ", step + 1, app.setup_state.total_steps);
    let bar_width = popup_width.saturating_sub(4) as usize;
    let filled_len = ((step + 1) as f32 / app.setup_state.total_steps as f32 * bar_width as f32) as usize;
    let filled_len = filled_len.min(bar_width); // safety
    
    let progress_bar_filled = "━".repeat(filled_len);
    let progress_bar_empty = "─".repeat(bar_width.saturating_sub(filled_len));

    let mut lines: Vec<Line> = Vec::new();

    // Progress bar
    lines.push(Line::from(vec![
        Span::styled(format!("  {}", progress_bar_filled), Style::default().fg(accent)),
        Span::styled(progress_bar_empty, Style::default().fg(comment)),
    ]));
    lines.push(Line::from(""));

    match step {
        0 => {
            // Step 1: Username
            lines.push(Line::from(Span::styled(
                "   What should we call you?",
                Style::default().fg(fg).add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  This will be used for your welcome greeting.",
                Style::default().fg(comment),
            )));
            lines.push(Line::from(""));

            let name_display = if app.setup_state.editing_field {
                format!("  ▸ {} █", app.setup_state.username)
            } else {
                format!("  ▸ {}", app.setup_state.username)
            };
            lines.push(Line::from(Span::styled(
                name_display,
                Style::default().fg(accent).add_modifier(Modifier::BOLD),
            )));

            lines.push(Line::from(""));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Press Enter to continue →",
                Style::default().fg(accent_dim),
            )));
        }
        1 => {
            // Step 2: Theme selection
            lines.push(Line::from(Span::styled(
                "   Choose your theme",
                Style::default().fg(fg).add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Use ↑↓ to browse. The background will preview each theme live!",
                Style::default().fg(comment),
            )));
            lines.push(Line::from(""));

            let theme_names = Theme::names();
            let visible_start = app.setup_state.selected_theme.saturating_sub(4);
            let visible_end = (visible_start + 9).min(theme_names.len());

            for i in visible_start..visible_end {
                let is_selected = i == app.setup_state.selected_theme;
                let prefix = if is_selected { "  ▸ " } else { "    " };
                let label = if i < 2 {
                    format!("{} ★", theme_names[i]) // Star for Aether themes
                } else {
                    theme_names[i].clone()
                };

                let style = if is_selected {
                    Style::default().fg(accent).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(fg)
                };

                lines.push(Line::from(Span::styled(
                    format!("{}{}", prefix, label),
                    style,
                )));
            }

            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Press Enter to continue →",
                Style::default().fg(accent_dim),
            )));
        }
        2 => {
            // Step 3: Edit mode
            lines.push(Line::from(Span::styled(
                "    Choose your editing mode",
                Style::default().fg(fg).add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));

            let modes = [
                ("Vim", "Modal editing with Normal/Insert/Command modes (hjkl navigation)"),
                ("Nano", "Simple editing — just start typing (Ctrl shortcuts)"),
                ("Emacs", "Classic extensibility — C-p/n/f/b navigation and M-x commands"),
                ("Aether", "Smart hybrid — direct typing + Alt shortcuts for power features"),
            ];

            for (i, (name, desc)) in modes.iter().enumerate() {
                let is_selected = i == app.setup_state.selected_mode;
                let prefix = if is_selected { "  ▸ " } else { "    " };

                let name_style = if is_selected {
                    Style::default().fg(accent).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(fg)
                };

                let desc_style = if is_selected {
                    Style::default().fg(accent_dim)
                } else {
                    Style::default().fg(comment)
                };

                let recommended = if i == 3 { " (recommended)" } else { "" };

                lines.push(Line::from(Span::styled(
                    format!("{}{}{}", prefix, name, recommended),
                    name_style,
                )));
                lines.push(Line::from(Span::styled(
                    format!("      {}", desc),
                    desc_style,
                )));
                lines.push(Line::from(""));
            }

            lines.push(Line::from(Span::styled(
                "  Press Enter to continue →",
                Style::default().fg(accent_dim),
            )));
        }
        3 => {
            // Step 4: AI support
            lines.push(Line::from(Span::styled(
                "  󰚩 Local AI Assistant",
                Style::default().fg(fg).add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Aether can use a local AI model for code assistance.",
                Style::default().fg(comment),
            )));
            lines.push(Line::from(Span::styled(
                "  No internet needed — runs entirely on your machine.",
                Style::default().fg(comment),
            )));
            lines.push(Line::from(""));

            let models = [
                "None (Skip for now)",
                "Ollama (codellama)",
                "Ollama (llama3)",
                "Ollama (starcoder2)",
            ];

            for (i, name) in models.iter().enumerate() {
                let is_selected = i == app.setup_state.ai_model_choice;
                let prefix = if is_selected { "  ▸ ◉ " } else { "    ○ " };

                let style = if is_selected {
                    Style::default().fg(accent).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(fg)
                };

                lines.push(Line::from(Span::styled(
                    format!("{}{}", prefix, name),
                    style,
                )));
            }

            lines.push(Line::from(""));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Press Enter to finish setup 🚀",
                Style::default().fg(accent).add_modifier(Modifier::BOLD),
            )));
        }
        4 => {
            // Step 5: Toggles
            lines.push(Line::from(Span::styled(
                "  󰒓 Extra Options",
                Style::default().fg(fg).add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Use ↑↓ to switch. Press Enter to toggle the highlighted option.",
                Style::default().fg(comment),
            )));
            lines.push(Line::from(""));

            let auto_update_status = if app.setup_state.enable_auto_update { "[x]" } else { "[ ]" };
            let mouse_status = if app.setup_state.enable_mouse { "[x]" } else { "[ ]" };

            let focus0 = app.setup_state.options_focus == 0;
            let focus1 = app.setup_state.options_focus == 1;

            lines.push(Line::from(vec![
                Span::styled(if focus0 { "  ▸ " } else { "    " }, Style::default().fg(fg)),
                Span::styled(format!("Auto-Updater {}", auto_update_status), if focus0 { Style::default().fg(accent).add_modifier(Modifier::BOLD) } else { Style::default().fg(fg) }),
            ]));
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled(if focus1 { "  ▸ " } else { "    " }, Style::default().fg(fg)),
                Span::styled(format!("Mouse Support {}", mouse_status), if focus1 { Style::default().fg(accent).add_modifier(Modifier::BOLD) } else { Style::default().fg(fg) }),
            ]));

            lines.push(Line::from(""));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Press Enter to finish setup 󰄬",
                Style::default().fg(accent_dim),
            )));
        }
        _ => {}
    }

    // Esc hint at bottom
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Esc: Skip setup",
        Style::default().fg(comment),
    )));

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}
