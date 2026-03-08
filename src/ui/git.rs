use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_git_status(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    frame.render_widget(Block::default().bg(Color::Rgb(10, 14, 23)), area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Help
        ])
        .split(area);

    // Title
    let title = Paragraph::new(" Aether Git Status ")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    frame.render_widget(title, layout[0]);

    // Content
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(layout[1]);

    // Unstaged list
    let unstaged_items: Vec<ListItem> = app.git_unstaged.iter().enumerate().map(|(i, f)| {
        let style = if app.git_focus == 0 && app.git_selected == i {
            Style::default().fg(Color::Rgb(0, 200, 255)).bg(Color::Rgb(30, 37, 53))
        } else {
            Style::default().fg(Color::Rgb(240, 244, 255))
        };
        ListItem::new(format!("  {}", f)).style(style)
    }).collect();

    let unstaged_block = Block::default()
        .title(" Unstaged Changes ")
        .borders(Borders::ALL)
        .border_style(if app.git_focus == 0 { Style::default().fg(Color::Rgb(59, 110, 248)) } else { Style::default().fg(Color::Rgb(55, 65, 81)) })
        .border_type(BorderType::Rounded);
    
    let unstaged_list = List::new(unstaged_items).block(unstaged_block);
    frame.render_widget(unstaged_list, inner_layout[0]);

    // Staged list
    let staged_items: Vec<ListItem> = app.git_staged.iter().enumerate().map(|(i, f)| {
        let style = if app.git_focus == 1 && app.git_selected == i {
            Style::default().fg(Color::Rgb(0, 200, 255)).bg(Color::Rgb(30, 37, 53))
        } else {
            Style::default().fg(Color::Rgb(240, 244, 255))
        };
        ListItem::new(format!("  {}", f)).style(style)
    }).collect();

    let staged_block = Block::default()
        .title(" Staged Changes ")
        .borders(Borders::ALL)
        .border_style(if app.git_focus == 1 { Style::default().fg(Color::Rgb(59, 110, 248)) } else { Style::default().fg(Color::Rgb(55, 65, 81)) })
        .border_type(BorderType::Rounded);
    
    let staged_list = List::new(staged_items).block(staged_block);
    frame.render_widget(staged_list, inner_layout[1]);

    // Help
    let help = Paragraph::new(" [Arrow Keys] Navigate  [Space] Stage/Unstage  [c] Commit  [P] Push  [Esc] Back ")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    frame.render_widget(help, layout[2]);
}
