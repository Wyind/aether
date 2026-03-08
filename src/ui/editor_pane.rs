use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;
use crate::syntax::{SyntaxHighlighter, get_file_extension};

pub fn draw_editor_pane(frame: &mut Frame, app: &mut App, area: Rect) {
    if app.documents.is_empty() { return; }

    let theme = &app.theme;
    let doc = &mut app.documents[app.active_tab];

    // Calculate gutter width
    let line_count = doc.buffer.line_count();
    let gutter_width = if app.show_line_numbers {
        (line_count.to_string().len() + 2) as u16
    } else {
        0
    };

    // Layout: [gutter | editor]
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(gutter_width),
            Constraint::Min(1),
        ])
        .split(area);

    let editor_area = layout[1];
    let viewport_height = editor_area.height as usize;
    let viewport_width = editor_area.width as usize;

    // Update scroll
    doc.cursor.update_scroll(viewport_height, viewport_width);
    doc.cursor.clamp(&doc.buffer);

    let scroll_row = doc.cursor.scroll_row;
    let scroll_col = doc.cursor.scroll_col;

    // Create syntax highlighter
    let highlighter = SyntaxHighlighter::new();
    let file_ext = get_file_extension(&doc.file_type);

    // Draw line numbers gutter
    if app.show_line_numbers {
        let mut gutter_lines: Vec<Line> = Vec::new();
        for i in 0..viewport_height {
            let line_num = scroll_row + i;
            if line_num < line_count {
                let num_str = format!("{:>width$} ", line_num + 1, width = gutter_width as usize - 1);
                let style = if line_num == doc.cursor.row {
                    Style::default().fg(theme.accent).bg(theme.active_line_bg).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme.gutter_fg).bg(theme.gutter_bg)
                };
                gutter_lines.push(Line::from(Span::styled(num_str, style)));
            } else {
                let empty = format!("{:>width$} ", "~", width = gutter_width as usize - 1);
                gutter_lines.push(Line::from(Span::styled(
                    empty,
                    Style::default().fg(theme.gutter_fg).bg(theme.gutter_bg),
                )));
            }
        }
        let gutter = Paragraph::new(gutter_lines)
            .style(Style::default().bg(theme.gutter_bg));
        frame.render_widget(gutter, layout[0]);
    }

    // Draw editor content with syntax highlighting
    let mut editor_lines: Vec<Line> = Vec::new();
    for i in 0..viewport_height {
        let line_num = scroll_row + i;
        if line_num < line_count {
            let line_text = doc.buffer.get_line(line_num);
            let display_text = if scroll_col < line_text.len() {
                &line_text[scroll_col..]
            } else {
                ""
            };

            let highlighted = highlighter.highlight_line(display_text, file_ext, theme);

            // Apply active line background
            if line_num == doc.cursor.row {
                let spans: Vec<Span> = highlighted.spans.into_iter().map(|span| {
                    let mut style = span.style;
                    style = style.bg(theme.active_line_bg);
                    Span::styled(span.content.to_string(), style)
                }).collect();

                // Pad the rest of the line with active line bg
                let content_len: usize = spans.iter().map(|s| s.content.len()).sum();
                let mut all_spans = spans;
                if content_len < viewport_width {
                    all_spans.push(Span::styled(
                        " ".repeat(viewport_width - content_len),
                        Style::default().bg(theme.active_line_bg),
                    ));
                }
                editor_lines.push(Line::from(all_spans));
            } else {
                editor_lines.push(highlighted);
            }
        } else {
            editor_lines.push(Line::from(Span::styled(
                "~".to_string(),
                Style::default().fg(theme.gutter_fg).bg(theme.bg),
            )));
        }
    }

    let editor = Paragraph::new(editor_lines)
        .style(Style::default().bg(theme.bg));
    frame.render_widget(editor, editor_area);

    // Position cursor
    let cursor_x = editor_area.x + (doc.cursor.col.saturating_sub(scroll_col)) as u16;
    let cursor_y = editor_area.y + (doc.cursor.row.saturating_sub(scroll_row)) as u16;
    if cursor_x < editor_area.x + editor_area.width && cursor_y < editor_area.y + editor_area.height {
        frame.set_cursor_position(Position::new(cursor_x, cursor_y));
    }
}
