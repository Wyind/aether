use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::{App, AppFocus};

pub fn draw_ai_sidebar(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    
    let block = Block::default()
        .title(" 󰚩 AI Assistant ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(if app.focus == AppFocus::AiPrompt { theme.accent } else { theme.border }))
        .bg(theme.bg);
    
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    // Chat history
    let mut messages = Vec::new();
    for msg in &app.ai_chat_history {
        let title = if msg.role == "user" { " You " } else { " AI " };
        let style = if msg.role == "user" {
            Style::default().fg(theme.accent_dim).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
        };
        
        messages.push(Line::from(vec![
            Span::styled(title, style),
        ]));
        
        for line in msg.content.lines() {
            messages.push(Line::from(format!("  {}", line)));
        }
        messages.push(Line::from(""));
    }

    if app.ai_generating && app.ai_chat_history.last().map(|m| m.content.is_empty()).unwrap_or(true) {
        messages.push(Line::from(Span::styled("  Thinking...", Style::default().fg(theme.comment).add_modifier(Modifier::ITALIC))));
    }

    let scroll = messages.len().saturating_sub(chunks[0].height as usize);
    let chat_para = Paragraph::new(messages)
        .wrap(Wrap { trim: false })
        .scroll((scroll as u16, 0));
    
    frame.render_widget(chat_para, chunks[0]);

    // Input field
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(if app.focus == AppFocus::AiPrompt { theme.accent } else { theme.comment }));
    
    let input_text = if app.ai_input_buffer.is_empty() && app.focus != AppFocus::AiPrompt {
        " Ask AI..."
    } else {
        &app.ai_input_buffer
    };
    
    let input_para = Paragraph::new(input_text)
        .block(input_block)
        .style(if app.ai_input_buffer.is_empty() { Style::default().fg(theme.comment) } else { Style::default().fg(theme.fg) });
    
    frame.render_widget(input_para, chunks[1]);
}
