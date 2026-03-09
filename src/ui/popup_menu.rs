use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_popup_menu(frame: &mut Frame, app: &App) {
    let popup = if let Some(p) = &app.active_popup { p } else { return };
    let theme = &app.theme;
    let area = frame.area();

    // Menu size calculation
    let max_opt_len = popup.options.iter().map(|s| s.len()).max().unwrap_or(20);
    let width = (max_opt_len + 10).min(area.width as usize - 4) as u16;
    
    // Max 12 rows visible
    let visible_height = 12.min(popup.options.len());
    let height = (visible_height + 2) as u16;

    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    let popup_area = Rect::new(x, y, width, height);

    let block = Block::default()
        .title(Span::styled(&popup.title, Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.popup_border))
        .style(Style::default().bg(theme.popup_bg));

    frame.render_widget(Clear, popup_area);
    frame.render_widget(&block, popup_area);

    let inner = block.inner(popup_area);
    
    let scroll = popup.scroll;
    let items: Vec<ListItem> = popup.options.iter().enumerate()
        .skip(scroll)
        .take(visible_height)
        .map(|(i, opt)| {
            let is_selected = i == popup.selected;
            let prefix = if is_selected { " ▸ " } else { "   " };
            
            let style = if is_selected {
                Style::default()
                    .fg(theme.accent)
                    .bg(theme.sidebar_active_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };

            ListItem::new(Span::styled(format!("{}{}", prefix, opt), style))
        }).collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}
