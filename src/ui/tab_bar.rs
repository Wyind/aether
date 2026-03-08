use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_tab_bar(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;

    if app.documents.is_empty() {
        let bar = Block::default().style(Style::default().bg(theme.tab_bg));
        frame.render_widget(bar, area);
        return;
    }

    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::styled(" ", Style::default().bg(theme.tab_bg)));

    for (i, doc) in app.documents.iter().enumerate() {
        let is_active = i == app.active_tab;
        let name = doc.file_name();
        let modified = if doc.modified { " ●" } else { "" };

        let tab_text = format!(" {}{} ", name, modified);

        let style = if is_active {
            Style::default()
                .fg(theme.tab_active_fg)
                .bg(theme.tab_active_bg)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(theme.tab_fg)
                .bg(theme.tab_bg)
        };

        if is_active {
            spans.push(Span::styled("▎", Style::default().fg(theme.accent).bg(theme.tab_active_bg)));
        }
        spans.push(Span::styled(tab_text, style));
        if is_active {
            spans.push(Span::styled("▕", Style::default().fg(theme.border).bg(theme.tab_bg)));
        } else {
            spans.push(Span::styled("│", Style::default().fg(theme.border).bg(theme.tab_bg)));
        }
    }

    // Fill remaining space
    let used_width: usize = spans.iter().map(|s| s.content.len()).sum();
    if (used_width as u16) < area.width {
        let remaining = area.width as usize - used_width;
        spans.push(Span::styled(
            " ".repeat(remaining),
            Style::default().bg(theme.tab_bg),
        ));
    }

    let line = Line::from(spans);
    let tabs = Paragraph::new(line);
    frame.render_widget(tabs, area);
}
