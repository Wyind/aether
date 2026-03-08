use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_about(frame: &mut Frame, app: &mut App) {
    let theme = &app.theme;
    let area = frame.area();

    // Fill background
    frame.render_widget(
        Block::default().style(Style::default().bg(theme.bg)),
        area,
    );

    let popup_width = 70;
    let popup_height = 20;
    let x = area.width.saturating_sub(popup_width) / 2;
    let y = area.height.saturating_sub(popup_height) / 2;
    let popup_area = Rect::new(x, y, popup_width.min(area.width), popup_height.min(area.height));

    let block = Block::default()
        .title(" About Aether ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(theme.status_bg).fg(theme.fg));

    let inner = block.inner(popup_area);
    frame.render_widget(Clear, popup_area);
    frame.render_widget(block, popup_area);

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(format!("Aether IDE v{}", include_str!("../../version.txt").trim()), Style::default().fg(theme.accent).add_modifier(Modifier::BOLD))),
        Line::from("A beautiful TUI text editor & IDE"),
        Line::from(""),
        Line::from(Span::styled("── Author ──", Style::default().fg(theme.accent_dim))),
        Line::from("Wyind"),
        Line::from(Span::styled("https://wyind.dev/", Style::default().fg(theme.string))),
        Line::from(Span::styled("https://github.com/Wyind", Style::default().fg(theme.string))),
        Line::from(""),
        Line::from(Span::styled("── Built With ──", Style::default().fg(theme.accent_dim))),
        Line::from("• ratatui - Terminal UI framework"),
        Line::from("• crossterm - Terminal backend"),
        Line::from("• syntect - Syntax highlighting"),
        Line::from("• mlua - Lua scripting engine"),
        Line::from("• ollama - Local AI integration"),
        Line::from(""),
        Line::from(Span::styled("Press Esc or Enter to return", Style::default().fg(theme.comment))),
    ];

    for line in &mut lines {
        if line.width() < inner.width as usize {
            line.alignment = Some(Alignment::Center);
        }
    }

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(paragraph, inner);
}
