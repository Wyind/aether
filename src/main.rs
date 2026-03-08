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
use ratatui::crossterm::event::Event;

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
        let mut a = App::new(config);
        for file in &files_to_open {
            a.open_file(file);
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
            } else if let Event::Mouse(mouse_event) = event::read()? {
                app.handle_mouse_event(mouse_event);
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
