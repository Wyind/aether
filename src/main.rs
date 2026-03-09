mod app;
mod editor;
mod input;
mod syntax;
mod theme;
mod ui;
mod config;
mod ai;
mod updater;
mod plugin;

use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use ratatui::crossterm::event::{Event, KeyCode};

use app::{App, AppScreen, AppFocus};

fn print_help() {
    println!("Aether — a beautiful TUI text editor & IDE");
    println!();
    println!("USAGE:");
    println!("    aether [OPTIONS] [FILE]...");
    println!();
    println!("OPTIONS:");
    println!("    --setup       Run the setup wizard again");
    println!("    --theme NAME  Start with a specific theme");
    println!("    --mode MODE   Start with a specific mode (vim/nano/aether)");
    println!("    --help, -h    Show this help message");
    println!("    --version, -v Show version information");
    println!();
    println!("EXAMPLES:");
    println!("    aether                  Open Aether (welcome screen)");
    println!("    aether main.rs          Open a file");
    println!("    aether src/*.rs         Open multiple files");
    println!("    aether --setup          Re-run the setup wizard");
    println!("    aether --theme Ember    Start with Ember theme");
}

fn print_version() {
    let version = include_str!("../version.txt").trim();
    println!("Aether v{}", version);
    println!("A beautiful TUI text editor & IDE");
    println!("https://github.com/wyind/aether");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Parse flags
    let mut force_setup = false;
    let mut theme_override: Option<String> = None;
    let mut mode_override: Option<String> = None;
    let mut files_to_open: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--version" | "-v" => {
                print_version();
                return Ok(());
            }
            "--setup" => {
                force_setup = true;
            }
            "--theme" => {
                i += 1;
                if i < args.len() {
                    theme_override = Some(args[i].clone());
                }
            }
            "--mode" => {
                i += 1;
                if i < args.len() {
                    mode_override = Some(args[i].clone());
                }
            }
            arg => {
                if !arg.starts_with('-') {
                    files_to_open.push(arg.to_string());
                }
            }
        }
        i += 1;
    }

    // Load config
    let mut config = config::Config::load_or_default();

    // Apply overrides
    if let Some(theme_name) = &theme_override {
        let themes = theme::Theme::all();
        if let Some(idx) = themes.iter().position(|t| t.name.to_lowercase() == theme_name.to_lowercase()) {
            config.theme_index = idx;
        }
    }
    if let Some(mode) = &mode_override {
        config.edit_mode = mode.to_lowercase();
    }

    let first_run = !config::Config::config_exists();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Auto Updater check no longer forces update on startup
    terminal.clear()?;

    // Create app
    let mut app = if force_setup || first_run {
        App::new_with_setup(config)
    } else if !files_to_open.is_empty() {
        let mut a = App::new_with_file(config, &files_to_open[0]);
        for i in 1..files_to_open.len() {
            a.open_file(&files_to_open[i]);
        }
        a.screen = AppScreen::Editor;
        a
    } else {
        App::new(config)
    };

    // Main loop
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {err}");
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        app.check_ai_rx();
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if app.handle_input(key) {
                    continue;
                }

                let is_ctrl = key.modifiers.contains(ratatui::crossterm::event::KeyModifiers::CONTROL);
                let is_alt = key.modifiers.contains(ratatui::crossterm::event::KeyModifiers::ALT);

                if is_ctrl || is_alt {
                    match key.code {
                        KeyCode::Char('x') if is_ctrl => { app.close_current_tab(); continue; }
                        KeyCode::Char('o') if is_ctrl => { app.open_file_picker(); continue; }
                        KeyCode::Char('n') if is_ctrl => { app.new_file(); continue; }
                        KeyCode::Char('p') if is_ctrl => { app.screen = AppScreen::CommandPalette; continue; }
                        KeyCode::Tab if is_ctrl => { app.next_tab(); continue; }
                        
                        // Tab switching (Ctrl+1-9 or Alt+1-9)
                        KeyCode::Char('1') => { app.select_tab(0); continue; }
                        KeyCode::Char('2') => { app.select_tab(1); continue; }
                        KeyCode::Char('3') => { app.select_tab(2); continue; }
                        KeyCode::Char('4') => { app.select_tab(3); continue; }
                        KeyCode::Char('5') => { app.select_tab(4); continue; }
                        KeyCode::Char('6') => { app.select_tab(5); continue; }
                        KeyCode::Char('7') => { app.select_tab(6); continue; }
                        KeyCode::Char('8') => { app.select_tab(7); continue; }
                        KeyCode::Char('9') => { app.select_tab(8); continue; }
                        
                        // Tab navigation (Alt+Left/Right)
                        KeyCode::Left if is_alt => {
                            if app.active_tab > 0 {
                                app.active_tab -= 1;
                            } else if !app.documents.is_empty() {
                                app.active_tab = app.documents.len() - 1;
                            }
                            continue;
                        }
                        KeyCode::Right if is_alt => {
                            app.next_tab();
                            continue;
                        }
                        _ => {}
                    }
                }

                if app.active_popup.is_some() {
                    app.handle_popup_input(key);
                } else if app.go_to_line_active {
                    app.handle_go_to_line_input(key);
                } else {
                    match app.screen {
                        AppScreen::Setup => app.handle_setup_input(key),
                        AppScreen::Welcome => app.handle_welcome_input(key),
                        AppScreen::Editor => {
                            if app.show_ai_sidebar && app.focus == AppFocus::AiPrompt {
                                app.handle_ai_input(key);
                            } else if app.file_picker_active {
                                app.handle_file_picker_input(key);
                            } else {
                                app.handle_editor_input(key);
                            }
                        }
                        AppScreen::CommandPalette => app.handle_command_palette_input(key),
                        AppScreen::About => app.handle_about_input(key),
                        AppScreen::Updater => app.handle_updater_input(key),
                        AppScreen::GitStatus => app.handle_git_status_input(key),
                        AppScreen::GitDiff => app.handle_git_diff_input(key),
                        AppScreen::GitCommit => app.handle_git_commit_input(key),
                        AppScreen::Controls => app.handle_controls_input(key),
                    }
                }
            }
 else if let Event::Mouse(mouse_event) = event::read()? {
                app.handle_mouse_event(mouse_event);
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
