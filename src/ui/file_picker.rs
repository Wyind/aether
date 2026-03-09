use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::*;

/// Built-in TUI file picker for opening files
pub fn draw_file_picker(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let area = frame.area();

    // Full-screen overlay
    let popup_width = 70.min(area.width.saturating_sub(4));
    let popup_height = 24.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    // Dim background
    let overlay = Block::default().style(Style::default().bg(Color::Rgb(0, 0, 0)));
    frame.render_widget(overlay, area);

    // Picker block
    let block = Block::default()
        .title(Span::styled(
            "  Open File ",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.popup_border))
        .style(Style::default().bg(theme.popup_bg));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    if !app.file_picker_active {
        return;
    }

    let fp = &app.file_picker_state;

    // Layout: [path bar, search, file list, hints]
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Current path
            Constraint::Length(2), // Search filter
            Constraint::Min(1),    // File list
            Constraint::Length(1), // Hints
        ])
        .split(inner);

    // Current path display
    let path_display = format!("   {}", fp.current_dir);
    let path_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.popup_bg));
    let path_inner = path_block.inner(layout[0]);
    frame.render_widget(path_block, layout[0]);
    frame.render_widget(
        Paragraph::new(path_display)
            .style(Style::default().fg(theme.accent_dim).bg(theme.popup_bg)),
        path_inner,
    );

    // Search filter input
    let search_display = if fp.filter_query.is_empty() {
        "   Type to filter...".to_string()
    } else {
        format!("   {}", fp.filter_query)
    };

    let search_style = if fp.filter_query.is_empty() {
        Style::default().fg(theme.comment).bg(theme.popup_bg)
    } else {
        Style::default().fg(theme.fg).bg(theme.popup_bg)
    };

    let search_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.popup_bg));
    let search_inner = search_block.inner(layout[1]);
    frame.render_widget(search_block, layout[1]);
    frame.render_widget(
        Paragraph::new(search_display).style(search_style),
        search_inner,
    );

    // File list
    let visible_height = layout[2].height as usize;
    let scroll = fp.scroll;
    let filtered = &fp.filtered_entries;
    let mut lines: Vec<Line> = Vec::new();

    for (display_i, entry_i) in filtered
        .iter()
        .enumerate()
        .skip(scroll)
        .take(visible_height)
    {
        let entry = &fp.entries[*entry_i];
        let is_selected = display_i == fp.selected;

        let icon = if entry.is_dir {
            " "
        } else {
            file_picker_icon(&entry.name)
        };

        let size_str = if entry.is_dir {
            "      ".to_string()
        } else {
            format_file_size(entry.size)
        };

        let name_width = popup_width.saturating_sub(16) as usize;

        let style = if is_selected {
            Style::default()
                .fg(theme.accent)
                .bg(theme.sidebar_active_bg)
                .add_modifier(Modifier::BOLD)
        } else if entry.is_dir {
            Style::default().fg(theme.accent_dim).bg(theme.popup_bg)
        } else {
            Style::default().fg(theme.fg).bg(theme.popup_bg)
        };

        let size_style = if is_selected {
            Style::default()
                .fg(theme.comment)
                .bg(theme.sidebar_active_bg)
        } else {
            Style::default().fg(theme.comment).bg(theme.popup_bg)
        };

        let prefix = if is_selected { " ▸ " } else { "   " };

        let line = Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(
                format!("{}{:<width$}", icon, entry.name, width = name_width),
                style,
            ),
            Span::styled(size_str, size_style),
        ]);
        lines.push(line);
    }

    // Fill remaining
    for _ in lines.len()..visible_height {
        lines.push(Line::from(Span::styled(
            " ".repeat(layout[2].width as usize),
            Style::default().bg(theme.popup_bg),
        )));
    }

    let list = Paragraph::new(lines);
    frame.render_widget(list, layout[2]);

    // Hints bar
    let hints = "  Enter: Open │ Backspace: Parent │ /: Filter │ Esc: Cancel";
    let hints_widget =
        Paragraph::new(hints).style(Style::default().fg(theme.comment).bg(theme.popup_bg));
    frame.render_widget(hints_widget, layout[3]);
}

fn file_picker_icon(name: &str) -> &'static str {
    let ext = name.rsplit('.').next().unwrap_or("");
    match ext {
        "rs" => " ",
        "py" => " ",
        "js" | "jsx" => " ",
        "ts" | "tsx" => " ",
        "html" | "htm" => " ",
        "css" | "scss" => " ",
        "json" => " ",
        "toml" | "yaml" | "yml" => " ",
        "md" => " ",
        "sh" | "bash" => " ",
        "lua" => " ",
        "go" => " ",
        "c" | "cpp" | "h" | "hpp" => " ",
        "png" | "jpg" | "gif" | "svg" => " ",
        "lock" => " ",
        _ => " ",
    }
}

fn format_file_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{:>5}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:>4.1}K", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:>4.1}M", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:>4.1}G", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}
