use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_file_tree(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;

    let block = Block::default()
        .title(Span::styled(
            " Files ",
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.sidebar_bg));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.file_tree_entries.is_empty() {
        let empty = Paragraph::new("  (empty)")
            .style(Style::default().fg(theme.gutter_fg).bg(theme.sidebar_bg));
        frame.render_widget(empty, inner);
        return;
    }

    let visible_height = inner.height as usize;
    let scroll = app.file_tree_scroll;

    let mut lines: Vec<Line> = Vec::new();
    for (i, entry) in app.file_tree_entries.iter().enumerate().skip(scroll).take(visible_height) {
        let indent = "  ".repeat(entry.depth);
        let icon = if entry.is_dir {
            if entry.expanded { "▼ " } else { "▶ " }
        } else {
            file_icon(&entry.name)
        };

        let style = if i == app.file_tree_selected {
            Style::default().fg(theme.accent).bg(theme.sidebar_active_bg).add_modifier(Modifier::BOLD)
        } else if entry.is_dir {
            Style::default().fg(theme.accent_dim).bg(theme.sidebar_bg)
        } else {
            Style::default().fg(theme.sidebar_fg).bg(theme.sidebar_bg)
        };

        let line_text = format!("{}{}{}", indent, icon, entry.name);
        // Pad to fill the width
        let padded = format!("{:<width$}", line_text, width = inner.width as usize);
        lines.push(Line::from(Span::styled(padded, style)));
    }

    // Fill remaining space
    for _ in lines.len()..visible_height {
        lines.push(Line::from(Span::styled(
            " ".repeat(inner.width as usize),
            Style::default().bg(theme.sidebar_bg),
        )));
    }

    let tree = Paragraph::new(lines);
    frame.render_widget(tree, inner);
}

fn file_icon(name: &str) -> &'static str {
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
        "java" => " ",
        "rb" => " ",
        "lock" => " ",
        "txt" => " ",
        "png" | "jpg" | "jpeg" | "gif" | "svg" => " ",
        _ => "  ",
    }
}
