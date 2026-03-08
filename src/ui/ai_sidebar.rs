use crate::app::{App, AppFocus};
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn draw_ai_sidebar(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;

    let block = Block::default()
        .title(" 󰚩 AI Assistant ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(if app.focus == AppFocus::AiPrompt {
            theme.accent
        } else {
            theme.border
        }))
        .bg(theme.bg);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(inner);

    // Chat history
    let mut messages = Vec::new();
    for msg in &app.ai_chat_history {
        let title = if msg.role == "user" { " You " } else { " AI " };
        let style = if msg.role == "user" {
            Style::default()
                .fg(theme.accent_dim)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD)
        };

        messages.push(Line::from(vec![Span::styled(title, style)]));

        // Wrap chat history lines
        // Since Paragraph doesn't easily expose wrapped lines to us here for direct push,
        // we'll rely on Paragraph's own wrapping when we render.
        for line in msg.content.lines() {
            messages.push(Line::from(format!("  {}", line)));
        }
        messages.push(Line::from(""));
    }

    if app.ai_generating
        && app
            .ai_chat_history
            .last()
            .map(|m| !m.content.is_empty())
            .unwrap_or(true)
    {
        // Only show thinking if the last message is not already being populated
    } else if app.ai_generating {
        messages.push(Line::from(Span::styled(
            "  Thinking...",
            Style::default()
                .fg(theme.comment)
                .add_modifier(Modifier::ITALIC),
        )));
    }

    let scroll = messages.len().saturating_sub(chunks[0].height as usize);
    let chat_para = Paragraph::new(messages)
        .wrap(Wrap { trim: true }) // Enable wrapping for history
        .scroll((scroll as u16, 0));

    frame.render_widget(chat_para, chunks[0]);

    // Pending Commands Overlay
    if !app.pending_ai_commands.is_empty() {
        let mut pending_rows = vec![Line::from(vec![Span::styled(
            " ⚠ Pending AI Actions:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )])];
        for (i, cmd) in app.pending_ai_commands.iter().enumerate() {
            let text = match cmd {
                crate::app::AgentCommand::Create { path, .. } => {
                    format!("  [{}] CREATE {}", i + 1, path)
                }
                crate::app::AgentCommand::Append { path, .. } => {
                    format!("  [{}] APPEND {}", i + 1, path)
                }
                crate::app::AgentCommand::Read { path } => format!("  [{}] READ {}", i + 1, path),
                crate::app::AgentCommand::Delete { path } => {
                    format!("  [{}] DELETE {}", i + 1, path)
                }
                crate::app::AgentCommand::Rename { old, new } => {
                    format!("  [{}] RENAME {} -> {}", i + 1, old, new)
                }
                crate::app::AgentCommand::List { path } => format!("  [{}] LIST {}", i + 1, path),
                crate::app::AgentCommand::Grep { pattern, path } => {
                    format!("  [{}] GREP {} in {}", i + 1, pattern, path)
                }
                crate::app::AgentCommand::Shell { command } => {
                    format!("  [{}] SHELL {}", i + 1, command)
                }
                crate::app::AgentCommand::Test { command } => {
                    format!("  [{}] TEST {}", i + 1, command)
                }
                crate::app::AgentCommand::Commit { message } => {
                    format!("  [{}] COMMIT \"{}\"", i + 1, message)
                }
                crate::app::AgentCommand::WebFetch { url } => {
                    format!("  [{}] WEBFETCH {}", i + 1, url)
                }
            };
            pending_rows.push(Line::from(Span::styled(
                text,
                Style::default().fg(Color::Yellow),
            )));
        }
        pending_rows.push(Line::from(""));
        pending_rows.push(Line::from(Span::styled(
            " Press Alt+Shift+Enter to Approve All ",
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )));

        let height = (pending_rows.len() as u16).min(chunks[0].height.saturating_sub(2));
        let pending_para = Paragraph::new(pending_rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .wrap(Wrap { trim: true });

        let pending_area = Rect::new(
            chunks[0].x + 1,
            chunks[0].y + chunks[0].height.saturating_sub(height + 1),
            chunks[0].width.saturating_sub(2),
            height,
        );
        frame.render_widget(Clear, pending_area);
        frame.render_widget(pending_para, pending_area);
    }

    // Input field
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(if app.focus == AppFocus::AiPrompt {
            theme.accent
        } else {
            theme.comment
        }));

    // Placeholder logic
    if app.ai_input_buffer.is_empty() && app.focus != AppFocus::AiPrompt {
        let placeholder = Paragraph::new(" Ask AI...")
            .block(input_block)
            .style(Style::default().fg(theme.comment));
        frame.render_widget(placeholder, chunks[1]);
    } else {
        let input_para = Paragraph::new(app.ai_input_buffer.as_str())
            .block(input_block)
            .wrap(Wrap { trim: false }) // Enable wrapping for input
            .style(if app.ai_input_buffer.is_empty() {
                Style::default().fg(theme.comment)
            } else {
                Style::default().fg(theme.fg)
            });
        frame.render_widget(input_para, chunks[1]);
    }

    // Set cursor position for AI prompt
    if app.focus == AppFocus::AiPrompt {
        // Calculate wrapped cursor position (simple approximation for now)
        let input_width = chunks[1].width.saturating_sub(2);
        if input_width > 0 {
            let cursor_x = (app.ai_input_buffer.len() % input_width as usize) as u16;
            let cursor_y = (app.ai_input_buffer.len() / input_width as usize) as u16;

            if cursor_y < chunks[1].height.saturating_sub(2) {
                frame.set_cursor_position(Position::new(
                    chunks[1].x + 1 + cursor_x,
                    chunks[1].y + 1 + cursor_y,
                ));
            }
        }
    }
}
