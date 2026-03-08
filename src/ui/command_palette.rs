use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_command_palette(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let area = frame.area();

    // Centered popup overlay
    let popup_width = 60.min(area.width.saturating_sub(4));
    let popup_height = 16.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 4; // Upper third
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    // Semi-transparent overlay effect (darken background)
    let overlay = Block::default()
        .style(Style::default().bg(Color::Rgb(0, 0, 0)));
    frame.render_widget(overlay, area);

    // Popup block
    let block = Block::default()
        .title(Span::styled(
            "  Command Palette ",
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.popup_border))
        .style(Style::default().bg(theme.popup_bg));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    // Layout: [search input, results]
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
        ])
        .split(inner);

    // Search input
    let search_display = if app.command_palette.query.is_empty() {
        "  Type to search commands...".to_string()
    } else {
        format!("  {}", app.command_palette.query)
    };

    let input_style = if app.command_palette.query.is_empty() {
        Style::default().fg(theme.comment).bg(theme.popup_bg)
    } else {
        Style::default().fg(theme.fg).bg(theme.popup_bg).add_modifier(Modifier::BOLD)
    };

    let search_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.popup_bg));
    let search_inner = search_block.inner(layout[0]);
    frame.render_widget(search_block, layout[0]);

    let input = Paragraph::new(search_display).style(input_style);
    frame.render_widget(input, search_inner);

    // Results list
    let visible_height = layout[1].height as usize;
    let palette = &app.command_palette;
    let mut lines: Vec<Line> = Vec::new();

    for (display_idx, &cmd_idx) in palette.filtered.iter().enumerate().take(visible_height) {
        let (name, desc) = &palette.commands[cmd_idx];
        let is_selected = display_idx == palette.selected;

        let style = if is_selected {
            Style::default().fg(theme.accent).bg(theme.sidebar_active_bg).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg).bg(theme.popup_bg)
        };

        let desc_style = if is_selected {
            Style::default().fg(theme.accent_dim).bg(theme.sidebar_active_bg)
        } else {
            Style::default().fg(theme.comment).bg(theme.popup_bg)
        };

        let prefix = if is_selected { " ▸ " } else { "   " };
        let name_width = popup_width.saturating_sub(6) as usize;

        let line = Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(format!("{:<width$}", name, width = name_width / 2), style),
            Span::styled(format!(" {}", desc), desc_style),
        ]);
        lines.push(line);
    }

    // Fill remaining space
    for _ in lines.len()..visible_height {
        lines.push(Line::from(Span::styled(
            " ".repeat(layout[1].width as usize),
            Style::default().bg(theme.popup_bg),
        )));
    }

    let results = Paragraph::new(lines);
    frame.render_widget(results, layout[1]);

    // Position cursor at search input
    frame.set_cursor_position(Position::new(
        search_inner.x + 2 + app.command_palette.query.len() as u16,
        search_inner.y,
    ));
}
