pub mod editor_pane;
pub mod file_tree;
pub mod status_bar;
pub mod tab_bar;
pub mod command_palette;
pub mod welcome;
pub mod setup;
pub mod file_picker;
pub mod about;
pub mod git;
pub mod git_diff;
pub mod git_commit;
pub mod ai_sidebar;
pub mod controls;

use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::{App, AppScreen};

pub fn draw(frame: &mut Frame, app: &mut App) {
    match app.screen {
        AppScreen::Setup => setup::draw_setup(frame, app),
        AppScreen::Welcome => welcome::draw_welcome(frame, app),
        AppScreen::About => about::draw_about(frame, app),
        AppScreen::Editor => {
            draw_editor(frame, app);
            if app.file_picker_active {
                file_picker::draw_file_picker(frame, app);
            }
        }
        AppScreen::CommandPalette => {
            draw_editor(frame, app);
            command_palette::draw_command_palette(frame, app);
        }
        AppScreen::Updater => crate::updater::draw_updater(frame, app),
        AppScreen::GitStatus => git::draw_git_status(frame, app),
        AppScreen::GitDiff => git_diff::draw_git_diff(frame, app),
        AppScreen::GitCommit => git_commit::draw_git_commit(frame, app),
        AppScreen::Controls => controls::draw_controls(frame, app),
    }
}

fn draw_editor(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let theme = &app.theme;

    // Fill background
    frame.render_widget(
        Block::default().style(Style::default().bg(theme.bg)),
        area,
    );

    // Main layout: [tab_bar, content, status_bar, hints_bar]
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Tab bar
            Constraint::Min(1),    // Content area
            Constraint::Length(1), // Status bar
            Constraint::Length(1), // Hints bar
        ])
        .split(area);

    // Draw tab bar
    tab_bar::draw_tab_bar(frame, app, main_layout[0]);

    // Content area: [file_tree | editor_pane | ai_sidebar]
    let mut constraints = Vec::new();
    if app.show_file_tree && !app.documents.is_empty() {
        constraints.push(Constraint::Length(app.file_tree_width));
    }
    constraints.push(Constraint::Min(1));
    if app.show_ai_sidebar {
        constraints.push(Constraint::Length(app.ai_sidebar_width));
    }

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(main_layout[1]);

    let mut current_idx = 0;
    if app.show_file_tree && !app.documents.is_empty() {
        file_tree::draw_file_tree(frame, app, content_layout[current_idx]);
        current_idx += 1;
    }
    
    draw_editor_content(frame, app, content_layout[current_idx]);
    current_idx += 1;

    if app.show_ai_sidebar {
        ai_sidebar::draw_ai_sidebar(frame, app, content_layout[current_idx]);
    }

    // Draw status bar
    status_bar::draw_status_bar(frame, app, main_layout[2]);

    // Draw hints bar
    draw_hints_bar(frame, app, main_layout[3]);

    // Draw search overlay if searching
    if app.searching {
        draw_search_bar(frame, app, area);
    }

    // Draw save prompt
    if app.save_prompt_active {
        draw_save_prompt(frame, app, area);
    }

    // Draw Lua info if enabled
    if app.show_lua_info {
        draw_lua_toolbar(frame, app, area);
    }
}

fn draw_editor_content(frame: &mut Frame, app: &mut App, area: Rect) {
    if app.documents.is_empty() {
        // Show a mini welcome when no docs are open but we're in editor mode
        let theme = &app.theme;
        let msg = Paragraph::new("  No files open. Press Ctrl+N:New or Ctrl+O:Open. Backspace:Welcome.")
            .style(Style::default().fg(theme.comment).bg(theme.bg));
        frame.render_widget(msg, area);
        return;
    }
    editor_pane::draw_editor_pane(frame, app, area);
}

fn draw_hints_bar(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let mut hints = match &app.edit_mode {
        crate::app::EditMode::Vim => {
            match app.vim_mode {
                crate::app::VimSubMode::Normal => " i:Insert  /:Search  ::Command  Ctrl+P:Palette  Ctrl+T:Tree  Ctrl+O:Open  Ctrl+X:Close  F5:Theme".to_string(),
                crate::app::VimSubMode::Insert => " Esc:Normal  Ctrl+S:Save  Ctrl+P:Palette  Ctrl+O:Open  Ctrl+X:Close".to_string(),
                crate::app::VimSubMode::Command => " Enter:Execute  Esc:Cancel".to_string(),
            }
        }
        crate::app::EditMode::Nano => " Ctrl+S:Save  Ctrl+X:Close  Ctrl+O:Open  Ctrl+P:Palette  Ctrl+T:Tree  F5:Theme".to_string(),
        crate::app::EditMode::Emacs => " C-p/n/f/b:Move  C-a/e:Line  C-k:Kill  C-y:Yank  C-s:Save  M-x:Palette".to_string(),
        crate::app::EditMode::Aether => " Alt+hjkl:Nav  Ctrl+S:Save  Ctrl+X:Close  Ctrl+O:Open  Ctrl+P:Palette  Ctrl+T:Tree  F5:Theme".to_string(),
    };

    if app.show_tab_switch_hint && !app.documents.is_empty() {
        hints.push_str("  Ctrl+Tab:Cycle  Ctrl+1-9:Tabs");
    }

    let bar = Paragraph::new(hints)
        .style(Style::default().fg(theme.accent_dim).bg(theme.status_bg));
    frame.render_widget(bar, area);
}

fn draw_search_bar(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let width = 42.min(area.width);
    let search_area = Rect::new(
        area.width.saturating_sub(width + 2),
        0,
        width,
        1,
    );

    let search_text = format!(" 🔍 {}", if app.search_query.is_empty() { "Type to search..." } else { &app.search_query });
    let bar = Paragraph::new(search_text)
        .style(Style::default()
            .fg(theme.fg)
            .bg(theme.popup_bg)
            .add_modifier(Modifier::BOLD));
    frame.render_widget(bar, search_area);
}

fn draw_save_prompt(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let width = 60.min(area.width);
    let popup_area = Rect::new(
        area.width.saturating_sub(width) / 2,
        area.height / 2,
        width,
        3,
    );

    let title = "  Save As... ";
    let content = if app.save_prompt_query.is_empty() { "filename.ext" } else { &app.save_prompt_query };
    
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(theme.popup_bg).fg(theme.accent));

    let bar = Paragraph::new(format!("  {}  ", content))
        .block(block)
        .style(Style::default().fg(theme.fg));
        
    frame.render_widget(ratatui::widgets::Clear, popup_area);
    frame.render_widget(bar, popup_area);
}

fn draw_lua_toolbar(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let text = format!(" LUA: {} ", app.last_lua_key);
    let width = (text.len() + 2) as u16;
    let lua_area = Rect::new(
        area.width.saturating_sub(width + 2),
        area.height.saturating_sub(3),
        width,
        1,
    );

    let bar = Paragraph::new(text)
        .style(Style::default().fg(Color::Rgb(0, 0, 0)).bg(theme.accent)
        .add_modifier(Modifier::BOLD));
    
    frame.render_widget(bar, lua_area);
}
