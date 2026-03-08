use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_git_diff(frame: &mut Frame, app: &mut App) {
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
    let title_text = if app.git_focus == 0 {
        format!(" Git Diff: {} (Unstaged) ", app.git_unstaged.get(app.git_selected).map(|s| &s[3..]).unwrap_or("Unknown"))
    } else {
        format!(" Git Diff: {} (Staged) ", app.git_staged.get(app.git_selected).map(|s| &s[3..]).unwrap_or("Unknown"))
    };

    let title = Paragraph::new(title_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(Color::Rgb(0, 200, 255))));
    frame.render_widget(title, layout[0]);

    // Content (Diff)
    let diff_items: Vec<ListItem> = app.git_diff_content.iter().map(|line| {
        let style = if line.starts_with('+') && !line.starts_with("+++") {
            Style::default().fg(Color::Rgb(80, 250, 123)) // Green for additions
        } else if line.starts_with('-') && !line.starts_with("---") {
            Style::default().fg(Color::Rgb(255, 85, 85)) // Red for deletions
        } else if line.starts_with("@@") {
            Style::default().fg(Color::Rgb(139, 233, 253)) // Cyan for hunks
        } else {
            Style::default().fg(Color::Rgb(240, 244, 255))
        };
        ListItem::new(line.clone()).style(style)
    }).collect();

    let diff_list = List::new(diff_items)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Changes "));
    frame.render_widget(diff_list, layout[1]);

    // Help
    let help = Paragraph::new(" [q/Esc] Back to Status ")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    frame.render_widget(help, layout[2]);
}
