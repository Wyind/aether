use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn draw_ai_menu(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let block = Block::default()
        .title(" AI Setup ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.popup_bg));
    frame.render_widget(block, area);

    let inner = block.inner(area);
    let models = [
        "None (Skip for now)",
        "Ollama (Local - CodeLlama)",
        "Ollama (Local - StarCoder)",
        "OpenAI (GPT-4)",
        "Anthropic (Claude)",
    ];

    let selected = app.setup_state.ai_model_choice;

    let lines: Vec<Line> = models
        .iter()
        .enumerate()
        .map(|(i, &model)| {
            let is_selected = i == selected;
            let prefix = if is_selected { " ▸ " } else { "   " };
            
            let style = if is_selected {
                Style::default()
                    .fg(theme.accent)
                    .bg(theme.sidebar_active_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg).bg(theme.popup_bg)
            };

            Line::from(Span::styled(format!("{}{}", prefix, model), style))
        })
        .collect();

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}
