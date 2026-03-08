use ratatui::prelude::*;
use ratatui::widgets::*;
use std::sync::mpsc;
use std::thread;
use std::process::{Command, Stdio};

pub enum UpdateState {
    CheckingNetwork,
    CheckingVersion,
    Cloning,
    Building,
    BuildingLine(String),
    Installing,
    Success,
    UpToDate,
    Failed(String),
}

pub fn start_updater(app: &mut crate::app::App) {
    let (tx, rx) = mpsc::channel();
    app.updater_rx = Some(rx);
    app.updater_status = "Starting update check...".to_string();
    app.updater_progress = 0;
    app.updater_completed = false;

    // Spawn updater thread
    thread::spawn(move || {
        // 1. Check network
        let _ = tx.send(UpdateState::CheckingNetwork);
        if !check_internet() {
            let _ = tx.send(UpdateState::Failed("No internet connection".into()));
            return;
        }

        // 1.5. Check Version
        let _ = tx.send(UpdateState::CheckingVersion);
        let current_version = env!("CARGO_PKG_VERSION");
        match ureq::get("https://raw.githubusercontent.com/Wyind/aether/master/version.txt").call() {
            Ok(response) => {
                if let Ok(mut text) = response.into_string() {
                    text = text.trim().to_string();
                    if text == current_version || text.is_empty() || text <= current_version.to_string() {
                        let _ = tx.send(UpdateState::UpToDate);
                        return;
                    }
                } else {
                    let _ = tx.send(UpdateState::Failed("Failed to parse version info".into()));
                    return;
                }
            }
            Err(_) => {
                let _ = tx.send(UpdateState::Failed("Failed to fetch remote version info".into()));
                return;
            }
        }

        // 2. Clone/Fetch
        let _ = tx.send(UpdateState::Cloning);
        let temp_dir = std::env::temp_dir().join("aether_update");
        let _ = std::fs::remove_dir_all(&temp_dir); // Cleanup previous

        let clone_status = Command::new("git")
            .arg("clone")
            .arg("https://github.com/Wyind/aether.git")
            .arg(&temp_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        if clone_status.is_err() || !clone_status.unwrap().success() {
            let _ = tx.send(UpdateState::Failed("Failed to clone repository".into()));
            return;
        }

        // 3. Build
        let _ = tx.send(UpdateState::Building);
        let mut build_cmd = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&temp_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        if let Ok(mut child) = build_cmd {
            use std::io::{BufRead, BufReader};
            let stderr = child.stderr.take().unwrap();
            let reader = BufReader::new(stderr);
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    let _ = tx.send(UpdateState::BuildingLine(line));
                }
            }
            
            let status = child.wait();
            if status.is_err() || !status.unwrap().success() {
                let _ = tx.send(UpdateState::Failed("Build failed".into()));
                return;
            }
        } else {
            let _ = tx.send(UpdateState::Failed("Failed to spawn build command".into()));
            return;
        }

        // 4. Install
        let _ = tx.send(UpdateState::Installing);
        let binary_name = if cfg!(windows) { "aether.exe" } else { "aether" };
        let source_bin = temp_dir.join(format!("target/release/{}", binary_name));
        
        let target_path = if cfg!(windows) {
            dirs::data_local_dir().map(|d| d.join("Aether").join("bin").join(binary_name))
        } else if cfg!(target_os = "macos") {
            dirs::home_dir().map(|h| h.join("bin").join(binary_name))
        } else {
            // Linux/default
            dirs::home_dir().map(|h| h.join(".local").join("bin").join(binary_name))
        };

        if let Some(path) = target_path {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if std::fs::copy(&source_bin, &path).is_ok() {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755));
                }
                let _ = tx.send(UpdateState::Success);
            } else {
                let _ = tx.send(UpdateState::Failed("Failed to copy binary. Permission denied?".into()));
            }
        } else {
            let _ = tx.send(UpdateState::Failed("Could not determine installation path".into()));
        }
    });
}

pub fn check_updater_status(app: &mut crate::app::App) {
    if let Some(rx) = &app.updater_rx {
        while let Ok(state) = rx.try_recv() {
            match state {
                UpdateState::CheckingNetwork => { app.updater_status = "Checking network...".to_string(); app.updater_progress = 10; }
                UpdateState::CheckingVersion => { app.updater_status = "Checking for updates...".to_string(); app.updater_progress = 20; }
                UpdateState::Cloning => { app.updater_status = "Downloading latest version...".to_string(); app.updater_progress = 40; }
                UpdateState::Building => { app.updater_status = "Compiling release build...".to_string(); app.updater_progress = 60; }
                UpdateState::BuildingLine(line) => {
                    app.updater_logs.push(line);
                    if app.updater_logs.len() > 100 {
                        app.updater_logs.remove(0);
                    }
                    app.updater_progress = (app.updater_progress + 1).min(85);
                }
                UpdateState::Installing => { app.updater_status = "Installing to local bin...".to_string(); app.updater_progress = 90; }
                UpdateState::Success => { 
                    app.updater_status = "Update complete! Press Enter to restart.".to_string(); 
                    app.updater_progress = 100;
                    app.updater_completed = true;
                }
                UpdateState::UpToDate => { 
                    app.updater_status = "Aether is already up to date! Press Enter to return.".to_string(); 
                    app.updater_progress = 100;
                    app.updater_completed = true;
                }
                UpdateState::Failed(err) => {
                    app.updater_status = format!("Update failed: {}. Press Enter to return.", err);
                    app.updater_progress = 100;
                    app.updater_completed = true;
                }
            }
        }
    }
}

pub fn draw_updater(frame: &mut Frame, app: &mut crate::app::App) {
    check_updater_status(app);

    let area = frame.area();
    frame.render_widget(Block::default().bg(Color::Rgb(10, 14, 23)), area);

    let popup_width = 80;
    let popup_height = 20;
    let x = area.width.saturating_sub(popup_width) / 2;
    let y = area.height.saturating_sub(popup_height) / 2;
    let popup_area = Rect::new(x, y, popup_width.min(area.width), popup_height.min(area.height));

    // Shadow
    let shadow_area = Rect::new(x + 2, y + 1, popup_area.width, popup_area.height);
    frame.render_widget(Block::default().bg(Color::Black), shadow_area);

    let block = Block::default()
        .title(" Aether Auto-Updater ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .bg(Color::Rgb(17, 24, 39))
        .fg(Color::Rgb(240, 244, 255));

    let inner = block.inner(popup_area);
    frame.render_widget(Clear, popup_area);
    frame.render_widget(block, popup_area);

    let mut rows = vec![
        Line::from(vec![
            Span::styled(format!("{:^width$}", app.updater_status, width = inner.width as usize), Style::default().add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
    ];

    // Show logs
    if !app.updater_logs.is_empty() {
        let max_lines = 10;
        let start = app.updater_logs.len().saturating_sub(max_lines);
        for (i, log) in app.updater_logs[start..].iter().enumerate() {
            let is_last = i == (app.updater_logs.len() - start - 1);
            let style = if is_last {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            rows.push(Line::from(Span::styled(format!("> {}", log), style)));
        }
    }

    let paragraph = Paragraph::new(rows).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, Rect::new(inner.x + 2, inner.y + 1, inner.width.saturating_sub(4), 12));

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Rgb(0, 200, 255)).bg(Color::Rgb(55, 65, 81)))
        .percent(app.updater_progress);
    
    let gauge_area = Rect::new(inner.x + 2, inner.y + inner.height.saturating_sub(4), inner.width.saturating_sub(4), 3);
    frame.render_widget(gauge, gauge_area);
}

fn check_internet() -> bool {
    let (cmd, args) = if cfg!(windows) {
        ("ping", vec!["-n", "1", "8.8.8.8"])
    } else {
        ("ping", vec!["-c", "1", "8.8.8.8"])
    };

    Command::new(cmd)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
