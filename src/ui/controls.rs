use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::{App, EditMode};

pub fn draw_controls(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let area = frame.area();

    // Fill background
    frame.render_widget(
        Block::default().style(Style::default().bg(theme.bg)),
        area,
    );

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // Content
            Constraint::Length(3), // Footer
        ])
        .margin(2)
        .split(area);

    // Header
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("   Keyboard Controls ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" — Current Mode: {:?}", app.edit_mode), Style::default().fg(theme.comment)),
        ]),
        Line::from(Span::styled("  Detailed guide for all commands and shortcuts", Style::default().fg(theme.comment))),
    ]);
    frame.render_widget(header, layout[0]);

    // Content
    let mut lines = Vec::new();
    
    // Global Controls
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("  ── GLOBAL CONTROLS ──", Style::default().fg(theme.accent_dim).add_modifier(Modifier::BOLD))));
    let global_keys = vec![
        ("Ctrl + P", "Open Command Palette (search all commands)"),
        ("Ctrl + O", "Open File Picker"),
        ("Ctrl + N", "Create New File"),
        ("Ctrl + T", "Toggle File Tree Sidebar"),
        ("Alt  + A", "Toggle AI Sidebar"),
        ("F5      ", "Cycle Through Themes"),
        ("Escape  ", "Exit current mode or close popup"),
        ("Ctrl + Tab", "Cycle through open tabs"),
        ("Ctrl + 1-9", "Switch to specific tab"),
    ];
    for (key, desc) in global_keys {
        lines.push(Line::from(vec![
            Span::styled(format!("    {:<12}", key), Style::default().fg(theme.accent)),
            Span::styled(format!(" - {}", desc), Style::default().fg(theme.fg)),
        ]));
    }

    // Mode Specific Controls
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(format!("  ── {:?} MODE CONTROLS ──", app.edit_mode), Style::default().fg(theme.accent_dim).add_modifier(Modifier::BOLD))));
    
    match app.edit_mode {
        EditMode::Vim => {
            lines.push(Line::from(Span::styled("    Normal Mode Keys:", Style::default().fg(theme.comment).add_modifier(Modifier::ITALIC))));
            let vim_normal = vec![
                ("h, j, k, l", "Move cursor Left, Down, Up, Right"),
                ("i / a     ", "Enter Insert mode / Append"),
                ("v         ", "Enter Visual mode (selection)"),
                (":         ", "Enter Command mode"),
                ("/         ", "Search within document"),
                ("u / Ctrl+R", "Undo / Redo"),
                ("x         ", "Delete character"),
                ("dd / yy   ", "Delete / Yank (Copy) whole line"),
                ("p         ", "Paste (Put)"),
            ];
            for (key, desc) in vim_normal {
                lines.push(Line::from(vec![
                    Span::styled(format!("    {:<12}", key), Style::default().fg(theme.accent)),
                    Span::styled(format!(" - {}", desc), Style::default().fg(theme.fg)),
                ]));
            }
        }
        EditMode::Nano => {
            let nano_keys = vec![
                ("Ctrl + S", "Save current file"),
                ("Ctrl + X", "Close current tab"),
                ("Ctrl + K", "Cut (Kill) current line"),
                ("Ctrl + U", "Uncut (Paste) line"),
                ("Ctrl + W", "Search (Where is)"),
                ("Ctrl + G", "Help (Get Help)"),
            ];
            for (key, desc) in nano_keys {
                lines.push(Line::from(vec![
                    Span::styled(format!("    {:<12}", key), Style::default().fg(theme.accent)),
                    Span::styled(format!(" - {}", desc), Style::default().fg(theme.fg)),
                ]));
            }
        }
        EditMode::Emacs => {
            let emacs_keys = vec![
                ("C-p, C-n", "Previous line, Next line"),
                ("C-b, C-f", "Backward char, Forward char"),
                ("C-a, C-e", "Beginning of line, End of line"),
                ("C-k     ", "Kill line"),
                ("C-y     ", "Yank (Paste)"),
                ("C-s     ", "Search (Is-search)"),
                ("C-x C-s ", "Save File"),
                ("M-x     ", "Command Palette"),
            ];
            for (key, desc) in emacs_keys {
                lines.push(Line::from(vec![
                    Span::styled(format!("    {:<12}", key), Style::default().fg(theme.accent)),
                    Span::styled(format!(" - {}", desc), Style::default().fg(theme.fg)),
                ]));
            }
        }
        EditMode::Aether => {
            let aether_keys = vec![
                ("Alt + h,j,k,l", "Quick movement"),
                ("Ctrl + S    ", "Save current file"),
                ("Ctrl + X    ", "Close tab"),
                ("Ctrl + D    ", "Duplicate line"),
                ("Ctrl + /    ", "Toggle Comment"),
            ];
            for (key, desc) in aether_keys {
                lines.push(Line::from(vec![
                    Span::styled(format!("    {:<12}", key), Style::default().fg(theme.accent)),
                    Span::styled(format!(" - {}", desc), Style::default().fg(theme.fg)),
                ]));
            }
        }
    }


    let content = Paragraph::new(lines).wrap(Wrap { trim: false });
    frame.render_widget(content, layout[1]);

    // Footer
    let footer = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Press ", Style::default().fg(theme.comment)),
            Span::styled("ESC", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(" or ", Style::default().fg(theme.comment)),
            Span::styled("ENTER", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(" to return to the Welcome screen.", Style::default().fg(theme.comment)),
        ]),
    ]).alignment(Alignment::Center);
    frame.render_widget(footer, layout[2]);
}
