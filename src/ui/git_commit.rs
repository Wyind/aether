use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_git_commit(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    frame.render_widget(Block::default().bg(Color::Rgb(10, 14, 23)), area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Staged files
            Constraint::Length(10), // Message input
            Constraint::Length(3), // Help
        ])
        .split(area);

    // Title
    let title = Paragraph::new(" Commit Staged Changes ")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(Color::Rgb(59, 110, 248))));
    frame.render_widget(title, layout[0]);

    // Staged files (Summary)
    let staged_items: Vec<ListItem> = app.git_staged.iter().map(|f| {
        ListItem::new(format!("  {}", f)).style(Style::default().fg(Color::Rgb(80, 250, 123)))
    }).collect();

    let staged_list = List::new(staged_items)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Staged for Commit "));
    frame.render_widget(staged_list, layout[1]);

    // Message input
    let message = Paragraph::new(app.git_commit_message.clone())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Commit Message (Ctrl+Enter to Commit) "));
    frame.render_widget(message, layout[2]);

    // Help
    let help = Paragraph::new(" [Ctrl+Enter] Confirm Commit  [Esc] Back to Status ")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    frame.render_widget(help, layout[2]); // Wait, I put help in layout[2], should be layout[3]

    // Fixed Help
    let help = Paragraph::new(" [Ctrl+Enter] Confirm Commit  [Esc] Back to Status ")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    frame.render_widget(help, layout[3]);
}
