use crate::config::Config;
use crate::editor::document::Document;
use crate::theme::Theme;
use crate::ai::AiAssistant;
use crate::plugin::PluginManager;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind, MouseButton};

#[derive(Debug, Clone, PartialEq)]
pub enum AppScreen {
    Setup,
    Welcome,
    Editor,
    CommandPalette,
    About,
    Updater,
    GitStatus,
    GitDiff,
    GitCommit,
    Controls,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EditMode {
    Vim,
    Nano,
    Aether,
    Emacs,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppFocus {
    Editor,
    FileTree,
    AiPrompt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VimSubMode {
    Normal,
    Insert,
    Command,
}

#[derive(Debug, Clone)]
pub struct SetupState {
    pub step: usize,
    pub total_steps: usize,
    pub username: String,
    pub selected_theme: usize,
    pub selected_mode: usize,
    pub ai_model_choice: usize,
    pub editing_field: bool,
    pub enable_auto_update: bool,
    pub enable_mouse: bool,
    pub options_focus: usize, // 0 for auto_update, 1 for mouse
}

impl SetupState {
    pub fn new() -> Self {
        let username = whoami::username();
        Self {
            step: 0,
            total_steps: 5,
            username,
            selected_theme: 0,
            selected_mode: 2, // Aether mode by default
            ai_model_choice: 0,
            editing_field: true,
            enable_auto_update: true,
            enable_mouse: true,
            options_focus: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WelcomeState {
    pub selected_option: usize,
    pub recent_files: Vec<String>,
}

impl WelcomeState {
    pub fn new(recent_files: Vec<String>) -> Self {
        Self {
            selected_option: 0,
            recent_files,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandPaletteState {
    pub query: String,
    pub selected: usize,
    pub commands: Vec<(String, String)>,
    pub filtered: Vec<usize>,
}

impl CommandPaletteState {
    pub fn new() -> Self {
        let commands = vec![
            ("Open File".to_string(), "Open a file from disk".to_string()),
            ("Save File".to_string(), "Save the current file".to_string()),
            ("Save As".to_string(), "Save current file with a new name".to_string()),
            ("Close Tab".to_string(), "Close the current tab".to_string()),
            ("New File".to_string(), "Create a new empty file".to_string()),
            ("Toggle File Tree".to_string(), "Show/hide the file tree sidebar".to_string()),
            ("Switch Theme".to_string(), "Cycle through available themes".to_string()),
            ("Switch to Vim Mode".to_string(), "Use Vim keybindings".to_string()),
            ("Switch to Nano Mode".to_string(), "Use Nano keybindings".to_string()),
            ("Switch to Aether Mode".to_string(), "Use Aether keybindings".to_string()),
            ("Toggle Line Numbers".to_string(), "Show/hide line numbers".to_string()),
            ("Go to Line".to_string(), "Jump to a specific line number".to_string()),
            ("Find & Replace".to_string(), "Search and replace text".to_string()),
            ("Toggle Word Wrap".to_string(), "Enable/disable word wrapping".to_string()),
            ("Ask AI (Complete)".to_string(), "Use local AI to complete the current code".to_string()),
            ("Ask AI (Explain)".to_string(), "Use local AI to explain the current line/selection".to_string()),
            ("Ask AI Assistant".to_string(), "Open sidebar and focus AI chat input".to_string()),
            ("Toggle AI Sidebar".to_string(), "Show or hide the AI assistant panel".to_string()),
            ("Git: Interactive Status".to_string(), "Open interactive Git status view".to_string()),
            ("Git: Status".to_string(), "Show minimal git status".to_string()),
            ("Git: Add All".to_string(), "Stage all file changes in cwd".to_string()),
            ("Git: Commit".to_string(), "Commit staged changes with default message".to_string()),
            ("Git: Push".to_string(), "Push commits to remote".to_string()),
            ("GitHub: Login".to_string(), "Sign into GitHub CLI via browser".to_string()),
            ("Toggle Lua Info".to_string(), "Show/hide last Lua key info".to_string()),
            ("Toggle Tab Hints".to_string(), "Show/hide tab switching shortcuts".to_string()),
            ("Open Controls".to_string(), "Show keyboard shortcuts for the current mode".to_string()),
            ("Quit".to_string(), "Exit Aether".to_string()),
        ];
        let filtered = (0..commands.len()).collect();
        Self {
            query: String::new(),
            selected: 0,
            commands,
            filtered,
        }
    }

    pub fn update_filter(&mut self) {
        use fuzzy_matcher::FuzzyMatcher;
        use fuzzy_matcher::skim::SkimMatcherV2;

        let matcher = SkimMatcherV2::default();
        if self.query.is_empty() {
            self.filtered = (0..self.commands.len()).collect();
        } else {
            let mut scored: Vec<(usize, i64)> = self.commands.iter().enumerate()
                .filter_map(|(i, (name, _))| {
                    matcher.fuzzy_match(name, &self.query).map(|score| (i, score))
                })
                .collect();
            scored.sort_by(|a, b| b.1.cmp(&a.1));
            self.filtered = scored.into_iter().map(|(i, _)| i).collect();
        }
        self.selected = 0;
    }
}

#[derive(Debug, Clone)]
pub struct FilePickerState {
    pub current_dir: String,
    pub entries: Vec<FilePickerEntry>,
    pub filtered_entries: Vec<usize>,
    pub selected: usize,
    pub scroll: usize,
    pub filter_query: String,
}

#[derive(Debug, Clone)]
pub struct FilePickerEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
}

impl FilePickerState {
    pub fn new() -> Self {
        let cwd = std::env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let mut state = Self {
            current_dir: cwd,
            entries: Vec::new(),
            filtered_entries: Vec::new(),
            selected: 0,
            scroll: 0,
            filter_query: String::new(),
        };
        state.refresh();
        state
    }

    pub fn refresh(&mut self) {
        self.entries.clear();
        self.filter_query.clear();
        self.selected = 0;
        self.scroll = 0;

        let Ok(read_dir) = std::fs::read_dir(&self.current_dir) else { return };
        let mut items: Vec<FilePickerEntry> = read_dir
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                !name.starts_with('.')
            })
            .map(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                let path = e.path().to_string_lossy().to_string();
                let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
                let size = e.metadata().map(|m| m.len()).unwrap_or(0);
                FilePickerEntry { name, path, is_dir, size }
            })
            .collect();

        items.sort_by(|a, b| {
            b.is_dir.cmp(&a.is_dir).then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });

        self.entries = items;
        self.update_filter();
    }

    pub fn update_filter(&mut self) {
        if self.filter_query.is_empty() {
            self.filtered_entries = (0..self.entries.len()).collect();
        } else {
            let query = self.filter_query.to_lowercase();
            self.filtered_entries = self.entries.iter().enumerate()
                .filter(|(_, e)| e.name.to_lowercase().contains(&query))
                .map(|(i, _)| i)
                .collect();
        }
        self.selected = 0;
        self.scroll = 0;
    }

    pub fn navigate_up(&mut self) {
        if let Some(parent) = std::path::Path::new(&self.current_dir).parent() {
            self.current_dir = parent.to_string_lossy().to_string();
            self.refresh();
        }
    }

    pub fn enter_selected(&mut self) -> Option<String> {
        let idx = self.filtered_entries.get(self.selected)?;
        let entry = self.entries.get(*idx)?.clone();
        if entry.is_dir {
            self.current_dir = entry.path;
            self.refresh();
            None
        } else {
            Some(entry.path)
        }
    }
}

pub struct App {
    pub screen: AppScreen,
    pub should_quit: bool,
    pub config: Config,
    pub documents: Vec<Document>,
    pub active_tab: usize,
    pub edit_mode: EditMode,
    pub vim_mode: VimSubMode,
    pub vim_command_buffer: String,
    pub theme: Theme,
    pub theme_index: usize,
    pub show_file_tree: bool,
    pub file_tree_width: u16,
    pub show_line_numbers: bool,
    pub status_message: String,
    pub status_message_time: std::time::Instant,
    pub setup_state: SetupState,
    pub welcome_state: WelcomeState,
    pub command_palette: CommandPaletteState,
    pub search_query: String,
    pub searching: bool,
    pub file_tree_entries: Vec<FileTreeEntry>,
    pub file_tree_selected: usize,
    pub file_tree_scroll: usize,
    pub word_wrap: bool,
    pub file_picker_active: bool,
    pub file_picker_state: FilePickerState,
    pub ai_assistant: AiAssistant,
    pub focus: AppFocus,
    pub expanded_dirs: std::collections::HashSet<String>,
    pub save_prompt_active: bool,
    pub save_prompt_query: String,
    pub updater_rx: Option<std::sync::mpsc::Receiver<crate::updater::UpdateState>>,
    pub updater_status: String,
    pub updater_progress: u16,
    pub updater_completed: bool,
    pub plugin_manager: PluginManager,
    pub git_unstaged: Vec<String>,
    pub git_staged: Vec<String>,
    pub git_selected: usize,
    pub git_focus: usize, // 0: unstaged, 1: staged
    pub git_commit_message: String,
    pub git_diff_content: Vec<String>,
    pub last_lua_key: String,
    pub show_lua_info: bool,
    pub show_tab_switch_hint: bool,
    pub show_ai_sidebar: bool,
    pub ai_sidebar_width: u16,
    pub ai_chat_history: Vec<crate::ai::AiMessage>,
    pub ai_input_buffer: String,
    pub ai_generating: bool,
    pub ai_rx: Option<std::sync::mpsc::Receiver<crate::ai::AiResponse>>,
}

#[derive(Debug, Clone)]
pub struct FileTreeEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub depth: usize,
    pub expanded: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        let theme_index = config.theme_index;
        let edit_mode = match config.edit_mode.as_str() {
            "vim" => EditMode::Vim,
            "nano" => EditMode::Nano,
            "emacs" => EditMode::Emacs,
            _ => EditMode::Aether,
        };
        let mut app = Self {
            screen: AppScreen::Welcome,
            should_quit: false,
            documents: vec![],
            active_tab: 0,
            edit_mode,
            vim_mode: VimSubMode::Normal,
            vim_command_buffer: String::new(),
            theme: Theme::all()[theme_index].clone(),
            theme_index,
            show_file_tree: true,
            file_tree_width: 25,
            show_line_numbers: true,
            status_message: String::new(),
            status_message_time: std::time::Instant::now(),
            setup_state: SetupState::new(),
            welcome_state: WelcomeState::new(config.recent_files.clone()),
            command_palette: CommandPaletteState::new(),
            search_query: String::new(),
            searching: false,
            file_tree_entries: Vec::new(),
            file_tree_selected: 0,
            file_tree_scroll: 0,
            word_wrap: false,
            file_picker_active: false,
            file_picker_state: FilePickerState::new(),
            ai_assistant: AiAssistant::new(crate::ai::AiConfig {
                enabled: config.ai_enabled,
                backend: if config.ai_enabled { crate::ai::AiBackend::Ollama } else { crate::ai::AiBackend::None },
                model_name: config.ai_model.clone(),
                endpoint: "http://localhost:11434".to_string(),
            }),
            focus: AppFocus::Editor,
            expanded_dirs: std::collections::HashSet::new(),
            save_prompt_active: false,
            save_prompt_query: String::new(),
            last_lua_key: String::new(),
            show_lua_info: config.show_lua_info,
            show_tab_switch_hint: config.show_tab_switch_hint,
            config,
            updater_rx: None,
            updater_status: "Starting update check...".to_string(),
            updater_progress: 0,
            updater_completed: false,
            plugin_manager: PluginManager::new().expect("Failed to initialize Lua"),
            git_unstaged: Vec::new(),
            git_staged: Vec::new(),
            git_selected: 0,
            git_focus: 0,
            git_commit_message: String::new(),
            git_diff_content: Vec::new(),
            show_ai_sidebar: false,
            ai_sidebar_width: 35,
            ai_chat_history: Vec::new(),
            ai_input_buffer: String::new(),
            ai_generating: false,
            ai_rx: None,
        };
        app.plugin_manager.setup_api().expect("Failed to setup Lua API");
        let _ = app.plugin_manager.load_plugins();
        app.refresh_git_status();
        app.refresh_file_tree();

        if app.config.ai_enabled {
            crate::ai::AiConfig::start_ollama();
        }

        app
    }

    pub fn new_with_setup(config: Config) -> Self {
        let mut app = Self::new(config);
        app.screen = AppScreen::Setup;
        app
    }

    pub fn set_status(&mut self, msg: &str) {
        self.status_message = msg.to_string();
        self.status_message_time = std::time::Instant::now();
    }

    pub fn open_file(&mut self, path: &str) {
        // Check if already open
        for (i, doc) in self.documents.iter().enumerate() {
            if doc.file_path.as_deref() == Some(path) {
                self.active_tab = i;
                self.set_status(&format!("Switched to {}", path));
                return;
            }
        }
        match Document::open(path) {
            Ok(doc) => {
                let name = doc.file_name().to_string();
                self.documents.push(doc);
                self.active_tab = self.documents.len() - 1;
                self.screen = AppScreen::Editor;
                self.set_status(&format!("Opened {}", name));
                // Add to recent files
                let path_str = path.to_string();
                self.config.recent_files.retain(|f| f != &path_str);
                self.config.recent_files.insert(0, path_str);
                if self.config.recent_files.len() > 10 {
                    self.config.recent_files.truncate(10);
                }
                let _ = self.config.save();
                // Call Lua hook
                let ptr = self as *mut _;
                let _ = self.plugin_manager.run_hook("on_open", ptr);
            }
            Err(e) => {
                self.set_status(&format!("Error opening file: {}", e));
            }
        }
    }

    pub fn new_file(&mut self) {
        self.documents.push(Document::new());
        self.active_tab = self.documents.len() - 1;
        self.screen = AppScreen::Editor;
        self.set_status("New file created");
    }

    pub fn save_current(&mut self) {
        if self.documents.is_empty() { return; }
        
        // Call Lua hook
        let ptr = self as *mut _;
        let _ = self.plugin_manager.run_hook("on_save", ptr);

        if self.documents[self.active_tab].file_path.is_none() {
            self.save_prompt_active = true;
            self.save_prompt_query.clear();
            return;
        }

        let doc = &mut self.documents[self.active_tab];
        match doc.save() {
            Ok(_) => {
                let name = doc.file_name().to_string();
                self.set_status(&format!("Saved {}", name));
            }
            Err(e) => {
                self.set_status(&format!("Error saving: {}", e));
            }
        }
    }

    pub fn close_current_tab(&mut self) {
        if self.documents.is_empty() { return; }
        self.documents.remove(self.active_tab);
        if self.active_tab > 0 && self.active_tab >= self.documents.len() {
            self.active_tab = self.documents.len() - 1;
        }
        if self.documents.is_empty() {
            self.open_file_picker();
        }
    }

    pub fn cycle_theme(&mut self) {
        let themes = Theme::all();
        self.theme_index = (self.theme_index + 1) % themes.len();
        self.theme = themes[self.theme_index].clone();
        self.config.theme_index = self.theme_index;
        let _ = self.config.save();
        self.set_status(&format!("Theme: {}", self.theme.name));
    }

    pub fn refresh_file_tree(&mut self) {
        let cwd = std::env::current_dir().unwrap_or_default();
        self.file_tree_entries = Self::build_file_tree(cwd.to_str().unwrap_or("."), 0, 2);
    }

    // ===== INPUT HANDLERS =====

    pub fn handle_setup_input(&mut self, key: KeyEvent) {
        let state = &mut self.setup_state;

        match key.code {
            KeyCode::Esc => {
                // Skip setup
                self.screen = AppScreen::Welcome;
                let _ = self.config.save();
            }
            KeyCode::Enter => {
                if state.step < state.total_steps - 1 {
                    if state.step == 0 {
                        state.editing_field = false;
                        self.config.username = state.username.clone();
                    }
                    state.step += 1;
                } else {
                    // Finish setup
                    self.config.username = state.username.clone();
                    self.config.theme_index = state.selected_theme;
                    self.config.edit_mode = match state.selected_mode {
                        0 => "vim".to_string(),
                        1 => "nano".to_string(),
                        2 => "emacs".to_string(),
                        _ => "aether".to_string(),
                    };
                    let (enabled, model) = match state.ai_model_choice {
                        1 => (true, "codellama"),
                        2 => (true, "llama3"),
                        3 => (true, "starcoder2"),
                        _ => (false, "none"),
                    };
                    self.config.ai_enabled = enabled;
                    self.config.ai_model = model.to_string();
                    self.config.auto_update = state.enable_auto_update;
                    self.config.mouse_support = state.enable_mouse;
                    self.config.first_run = false;
                    let _ = self.config.save();

                    // Start/Install AI and update assistant
                    if enabled {
                        crate::ai::AiConfig::start_ollama();
                        crate::ai::AiConfig::pull_ollama_model(model);
                        self.ai_assistant = crate::ai::AiAssistant::new(crate::ai::AiConfig {
                            enabled,
                            backend: crate::ai::AiBackend::Ollama,
                            model_name: model.to_string(),
                            endpoint: "http://localhost:11434".to_string(),
                        });
                    }

                    // Apply settings
                    self.theme_index = state.selected_theme;
                    self.theme = Theme::all()[self.theme_index].clone();
                    self.edit_mode = match state.selected_mode {
                        0 => EditMode::Vim,
                        1 => EditMode::Nano,
                        2 => EditMode::Emacs,
                        _ => EditMode::Aether,
                    };

                    self.screen = AppScreen::Welcome;
                }
            }
            KeyCode::Tab | KeyCode::Down | KeyCode::Right => {
                if state.step == 0 {
                    state.editing_field = false;
                } else if state.step == 1 {
                    state.selected_theme = (state.selected_theme + 1) % Theme::all().len();
                } else if state.step == 2 {
                    state.selected_mode = (state.selected_mode + 1) % 4;
                } else if state.step == 3 {
                    state.ai_model_choice = (state.ai_model_choice + 1) % 4;
                } else if state.step == 4 {
                    // Switch focus
                    state.options_focus = (state.options_focus + 1) % 2;
                }
            }
            KeyCode::BackTab | KeyCode::Up | KeyCode::Left => {
                if state.step == 1 {
                    let len = Theme::all().len();
                    state.selected_theme = if state.selected_theme == 0 { len - 1 } else { state.selected_theme - 1 };
                } else if state.step == 2 {
                    state.selected_mode = if state.selected_mode == 0 { 3 } else { state.selected_mode - 1 };
                } else if state.step == 3 {
                    state.ai_model_choice = if state.ai_model_choice == 0 { 3 } else { state.ai_model_choice - 1 };
                } else if state.step == 4 {
                    // Switch focus
                    state.options_focus = if state.options_focus == 0 { 1 } else { 0 };
                }
            }
            KeyCode::Char(' ') => {
                if state.step == 4 {
                    // Toggle the currently focused option
                    if state.options_focus == 0 {
                        state.enable_auto_update = !state.enable_auto_update;
                    } else {
                        state.enable_mouse = !state.enable_mouse;
                    }
                }
            }
            KeyCode::Char(c) => {
                if state.step == 0 && state.editing_field {
                    state.username.push(c);
                }
            }
            KeyCode::Backspace => {
                if state.step == 0 && state.editing_field {
                    state.username.pop();
                }
                if state.step == 0 && !state.editing_field {
                    state.editing_field = true;
                }
            }
            _ => {}
        }
    }

    pub fn handle_welcome_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.new_file();
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                self.screen = AppScreen::About;
            }
            KeyCode::Char('o') | KeyCode::Char('O') => {
                self.open_file_picker();
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.screen = AppScreen::Controls;
            }
            KeyCode::Up | KeyCode::Left => {
                if self.welcome_state.selected_option > 0 {
                    self.welcome_state.selected_option -= 1;
                }
            }
            KeyCode::Down | KeyCode::Right => {
                let max = 4 + self.welcome_state.recent_files.len();
                if self.welcome_state.selected_option < max {
                    self.welcome_state.selected_option += 1;
                }
            }
            KeyCode::Enter => {
                match self.welcome_state.selected_option {
                    0 => self.new_file(),
                    1 => self.open_file_picker(),
                    2 => self.screen = AppScreen::Controls,
                    3 => self.screen = AppScreen::About,
                    4 => {
                        self.screen = AppScreen::Setup;
                        self.setup_state = SetupState::new();
                        self.setup_state.username = self.config.username.clone();
                    }
                    5 => {
                        if self.config.auto_update {
                            self.screen = AppScreen::Updater;
                            crate::updater::start_updater(self);
                        } else {
                            self.should_quit = true;
                        }
                    }
                    6 => self.should_quit = true,
                    n => {
                        let offset = if self.config.auto_update { 7 } else { 6 };
                        let idx = n.saturating_sub(offset);
                        if idx < self.welcome_state.recent_files.len() {
                            let path = self.welcome_state.recent_files[idx].clone();
                            self.open_file(&path);
                        }
                    }
                }
            }
            KeyCode::Char('u') | KeyCode::Char('U') => {
                if self.config.auto_update {
                    self.screen = AppScreen::Updater;
                    crate::updater::start_updater(self);
                }
            }
            KeyCode::Char('s') => {
                self.screen = AppScreen::Setup;
                self.setup_state = SetupState::new();
                self.setup_state.username = self.config.username.clone();
            }
            _ => {}
        }
    }

    pub fn handle_about_input(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Enter || key.code == KeyCode::Esc {
            self.screen = AppScreen::Welcome;
        }
    }

    pub fn handle_ai_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                if self.show_ai_sidebar {
                    self.show_ai_sidebar = false;
                }
                self.focus = AppFocus::Editor;
            }
            KeyCode::Enter => {
                if !self.ai_input_buffer.is_empty() && !self.ai_generating {
                    let prompt = self.ai_input_buffer.clone();
                    self.ai_input_buffer.clear();
                    self.send_ai_message(&prompt);
                }
            }
            KeyCode::Char(c) => {
                self.ai_input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.ai_input_buffer.pop();
            }
            _ => {}
        }
    }

    pub fn send_ai_message(&mut self, prompt: &str) {
        self.ai_chat_history.push(crate::ai::AiMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        });
        self.ai_chat_history.push(crate::ai::AiMessage {
            role: "assistant".to_string(),
            content: String::new(),
        });
        self.ai_generating = true;
        
        let (tx, rx) = std::sync::mpsc::channel();
        self.ai_rx = Some(rx);
        
        self.ai_assistant.chat(self.ai_chat_history.clone(), tx);
    }

    pub fn check_ai_rx(&mut self) {
        let mut should_clear = false;
        let mut full_text = None;

        if let Some(rx) = &self.ai_rx {
            while let Ok(response) = rx.try_recv() {
                match response {
                    crate::ai::AiResponse::Partial(text) => {
                        if let Some(msg) = self.ai_chat_history.last_mut() {
                            if msg.role == "assistant" {
                                msg.content.push_str(&text);
                            }
                        }
                    }
                    crate::ai::AiResponse::Full(text) => {
                        if let Some(msg) = self.ai_chat_history.last_mut() {
                            if msg.role == "assistant" {
                                msg.content = text.clone();
                            }
                        }
                        full_text = Some(text);
                        self.ai_generating = false;
                        should_clear = true;
                    }
                    crate::ai::AiResponse::Error(err) => {
                        if let Some(msg) = self.ai_chat_history.last_mut() {
                            if msg.role == "assistant" {
                                msg.content = format!("Error: {}", err);
                            }
                        }
                        self.ai_generating = false;
                        should_clear = true;
                    }
                }
            }
        }

        if let Some(text) = full_text {
            self.process_ai_agent_commands(&text);
        }

        if should_clear {
            self.ai_rx = None;
        }
    }

    fn process_ai_agent_commands(&mut self, text: &str) {
        use std::fs;
        use std::path::Path;

        // Simple tag-based parser
        let mut lines = text.lines();
        while let Some(line) = lines.next() {
            if line.starts_with("@@CREATE ") {
                let path_str = line.trim_start_matches("@@CREATE ").trim();
                let mut content = String::new();
                for content_line in lines.by_ref() {
                    if content_line.starts_with("@@") {
                        break;
                    }
                    content.push_str(content_line);
                    content.push('\n');
                }
                
                let path = Path::new(path_str);
                if let Some(parent) = path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                if fs::write(path, content).is_ok() {
                    self.set_status(&format!("AI Agent: Created {}", path_str));
                    self.refresh_file_tree();
                }
            } else if line.starts_with("@@APPEND ") {
                let path_str = line.trim_start_matches("@@APPEND ").trim();
                let mut content = String::new();
                for content_line in lines.by_ref() {
                    if content_line.starts_with("@@") {
                        break;
                    }
                    content.push_str(content_line);
                    content.push('\n');
                }
                
                if let Ok(mut existing) = fs::read_to_string(path_str) {
                    existing.push_str(&content);
                    if fs::write(path_str, existing).is_ok() {
                        self.set_status(&format!("AI Agent: Updated {}", path_str));
                    }
                }
            }
        }
    }

    pub fn handle_controls_input(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Esc || key.code == KeyCode::Enter {
            self.screen = AppScreen::Welcome;
        }
    }

    pub fn handle_updater_input(&mut self, key: KeyEvent) {
        if self.updater_completed && key.code == KeyCode::Enter {
            self.screen = AppScreen::Welcome;
            self.updater_rx = None;
        } else if key.code == KeyCode::Esc {
            self.screen = AppScreen::Welcome;
            self.updater_rx = None;
        }
    }

    pub fn handle_git_status_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.screen = AppScreen::Welcome;
            }
            KeyCode::Up => {
                if self.git_selected > 0 {
                    self.git_selected -= 1;
                }
            }
            KeyCode::Down => {
                let max = if self.git_focus == 0 { self.git_unstaged.len() } else { self.git_staged.len() };
                if self.git_selected + 1 < max {
                    self.git_selected += 1;
                }
            }
            KeyCode::Tab => {
                self.git_focus = 1 - self.git_focus;
                self.git_selected = 0;
            }
            KeyCode::Enter => {
                // Enter Diff view
                self.refresh_git_diff();
                if !self.git_diff_content.is_empty() {
                    self.screen = AppScreen::GitDiff;
                } else {
                    self.set_status("No changes to show");
                }
            }
            KeyCode::Char(' ') => {
                // Stage or unstage
                if self.git_focus == 0 {
                    if let Some(file_row) = self.git_unstaged.get(self.git_selected) {
                        let file = &file_row[3..];
                        let _ = std::process::Command::new("git").arg("add").arg(file).status();
                    }
                } else {
                    if let Some(file_row) = self.git_staged.get(self.git_selected) {
                        let file = &file_row[3..];
                        let _ = std::process::Command::new("git").arg("restore").arg("--staged").arg(file).status();
                    }
                }
                self.refresh_git_status();
            }
            KeyCode::Char('c') => {
                // Open commit screen if there are staged changes
                if !self.git_staged.is_empty() {
                    self.git_commit_message.clear();
                    self.screen = AppScreen::GitCommit;
                } else {
                    self.set_status("No staged changes to commit");
                }
            }
            KeyCode::Char('P') => {
                let _ = std::process::Command::new("git").arg("push").status();
                self.set_status("Git: Pushed to remote");
                self.refresh_git_status();
            }
            _ => {}
        }
    }

    pub fn handle_git_diff_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.screen = AppScreen::GitStatus;
            }
            KeyCode::Up => {
                // Diff scroll up
            }
            KeyCode::Down => {
                // Diff scroll down
            }
            _ => {}
        }
    }

    pub fn handle_git_commit_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.screen = AppScreen::GitStatus;
            }
            KeyCode::Enter => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.execute_git_commit();
                } else {
                    self.git_commit_message.push('\n');
                }
            }
            KeyCode::Char(c) => {
                self.git_commit_message.push(c);
            }
            KeyCode::Backspace => {
                self.git_commit_message.pop();
            }
            _ => {}
        }
    }

    pub fn handle_mouse_event(&mut self, event: MouseEvent) {
        if !self.config.mouse_support {
            return;
        }

        if event.kind == MouseEventKind::Down(MouseButton::Left) {
            match self.screen {
                AppScreen::Welcome => self.handle_welcome_mouse(event.column, event.row),
                _ => {}
            }
        }
    }

    pub fn handle_welcome_mouse(&mut self, _col: u16, row: u16) {
        if let Ok((_width, height)) = ratatui::crossterm::terminal::size() {
            let center_y = height / 2;
            let logo_height = 8;
            let content_height = logo_height + 15;
            let start_y = center_y.saturating_sub(content_height / 2);
            
            let options_start = start_y + 13;
            let options_count = if self.config.auto_update { 6 } else { 5 };
            let options_end = options_start + options_count;
            
            if row >= options_start && row < options_end {
                self.welcome_state.selected_option = (row - options_start) as usize;
                // Treat mouse click as Enter
                self.handle_welcome_input(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()));
            }
            
            // Recent files
            if !self.welcome_state.recent_files.is_empty() {
                let options_count = if self.config.auto_update { 6 } else { 5 };
                let recent_start = start_y + 21;
                let recent_end = recent_start + self.welcome_state.recent_files.len().min(5) as u16;
                
                if row >= recent_start && row < recent_end {
                    self.welcome_state.selected_option = options_count as usize + (row - recent_start) as usize;
                    self.handle_welcome_input(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()));
                }
            }
        }
    }

    pub fn handle_editor_input(&mut self, key: KeyEvent) {
        if self.save_prompt_active {
            match key.code {
                KeyCode::Esc => { self.save_prompt_active = false; }
                KeyCode::Enter => {
                    if !self.save_prompt_query.is_empty() {
                        let path = self.save_prompt_query.clone();
                        let doc = &mut self.documents[self.active_tab];
                        if doc.save_as(&path).is_ok() {
                            self.set_status(&format!("Saved {}", path));
                        } else {
                            self.set_status("Error saving file");
                        }
                        self.save_prompt_active = false;
                        self.refresh_file_tree();
                    }
                }
                KeyCode::Char(c) => { self.save_prompt_query.push(c); }
                KeyCode::Backspace => { self.save_prompt_query.pop(); }
                _ => {}
            }
            return;
        }

        // Global keybindings (work in all modes)
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('p') => {
                    self.command_palette = CommandPaletteState::new();
                    self.screen = AppScreen::CommandPalette;
                    return;
                }
                KeyCode::Char('t') => {
                    if self.show_file_tree {
                        if self.focus == AppFocus::FileTree {
                            self.show_file_tree = false;
                            self.focus = AppFocus::Editor;
                        } else {
                            self.focus = AppFocus::FileTree;
                        }
                    } else {
                        self.show_file_tree = true;
                        self.focus = AppFocus::FileTree;
                    }
                    return;
                }
                KeyCode::Char('s') => {
                    self.save_current();
                    return;
                }
                KeyCode::Char('w') | KeyCode::Char('x') => {
                    self.close_current_tab();
                    return;
                }
                KeyCode::Char('q') => {
                    self.should_quit = true;
                    return;
                }
                KeyCode::Char('n') => {
                    self.new_file();
                    return;
                }
                KeyCode::Char('o') => {
                    self.open_file_picker();
                    return;
                }
                KeyCode::Backspace if self.documents.is_empty() => {
                    self.screen = AppScreen::Welcome;
                    return;
                }
                KeyCode::Char('f') => {
                    self.searching = !self.searching;
                    if self.searching {
                        self.search_query.clear();
                    }
                    return;
                }
                KeyCode::Tab => {
                    if !self.documents.is_empty() {
                        self.active_tab = (self.active_tab + 1) % self.documents.len();
                    }
                    return;
                }
                KeyCode::Char(c) if c >= '1' && c <= '9' => {
                    let idx = (c as u8 - b'1') as usize;
                    if idx < self.documents.len() {
                        self.active_tab = idx;
                    }
                    return;
                }
                _ => {}
            }
        }

        // F5 to cycle theme
        if key.code == KeyCode::F(5) {
            self.cycle_theme();
            return;
        }

        // Alt+x for Palette in Emacs mode
        if key.modifiers.contains(KeyModifiers::ALT) && key.code == KeyCode::Char('x') && self.edit_mode == EditMode::Emacs {
            self.command_palette = CommandPaletteState::new();
            self.screen = AppScreen::CommandPalette;
            return;
        }

        // Alt+A to toggle AI Sidebar
        if key.modifiers.contains(KeyModifiers::ALT) && key.code == KeyCode::Char('a') {
            self.show_ai_sidebar = !self.show_ai_sidebar;
            if self.show_ai_sidebar {
                self.focus = AppFocus::AiPrompt;
            } else if self.focus == AppFocus::AiPrompt {
                self.focus = AppFocus::Editor;
            }
            return;
        }

        // File tree focus input
        if self.focus == AppFocus::FileTree {
            self.handle_file_tree_input(key);
            return;
        }

        // Search mode input
        if self.searching {
            match key.code {
                KeyCode::Esc => { self.searching = false; }
                KeyCode::Enter => {
                    // Find next occurrence
                    if !self.documents.is_empty() && !self.search_query.is_empty() {
                        let doc = &mut self.documents[self.active_tab];
                        doc.find_next(&self.search_query);
                    }
                }
                KeyCode::Char(c) => { self.search_query.push(c); }
                KeyCode::Backspace => { self.search_query.pop(); }
                _ => {}
            }
            return;
        }

        // Mode-specific input
        match self.edit_mode {
            EditMode::Vim => self.handle_vim_input(key),
            EditMode::Nano => self.handle_nano_input(key),
            EditMode::Aether => self.handle_aether_input(key),
            EditMode::Emacs => self.handle_emacs_input(key),
        }
    }

    pub fn handle_file_tree_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => { self.focus = AppFocus::Editor; }
            KeyCode::Up => {
                if self.file_tree_selected > 0 {
                    self.file_tree_selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.file_tree_selected + 1 < self.file_tree_entries.len() {
                    self.file_tree_selected += 1;
                }
            }
            KeyCode::Enter | KeyCode::Right => {
                if self.file_tree_entries.is_empty() { return; }
                let entry = self.file_tree_entries[self.file_tree_selected].clone();
                if entry.is_dir {
                    if self.expanded_dirs.contains(&entry.path) {
                        self.expanded_dirs.remove(&entry.path);
                    } else {
                        self.expanded_dirs.insert(entry.path);
                    }
                    self.refresh_file_tree();
                } else {
                    self.open_file(&entry.path);
                    self.focus = AppFocus::Editor;
                }
            }
            KeyCode::Left => {
                if self.file_tree_entries.is_empty() { return; }
                let entry = self.file_tree_entries[self.file_tree_selected].clone();
                if entry.is_dir && self.expanded_dirs.contains(&entry.path) {
                    self.expanded_dirs.remove(&entry.path);
                    self.refresh_file_tree();
                }
            }
            _ => {
                // Ignore other keys while focused on tree, or handle global
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    // Temporarily defocus to utilize global shorts 
                    self.focus = AppFocus::Editor;
                    self.handle_editor_input(key);
                    self.focus = AppFocus::FileTree;
                }
            }
        }
    }

    fn handle_vim_input(&mut self, key: KeyEvent) {
        // Forward to Lua plugins first
        let key_str = format!("{:?}", key.code);
        self.last_lua_key = key_str.clone();
        let ptr = self as *mut _;
        if let Ok(true) = self.plugin_manager.run_keypress_hook(&key_str, ptr) {
            return;
        }

        if self.documents.is_empty() { return; }

        match self.vim_mode {
            VimSubMode::Normal => {
                match key.code {
                    KeyCode::Char('i') => { self.vim_mode = VimSubMode::Insert; }
                    KeyCode::Char('a') => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.cursor.move_right(&doc.buffer);
                        self.vim_mode = VimSubMode::Insert;
                    }
                    KeyCode::Char('o') => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.insert_line_below();
                        self.vim_mode = VimSubMode::Insert;
                    }
                    KeyCode::Char('O') => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.insert_line_above();
                        self.vim_mode = VimSubMode::Insert;
                    }
                    KeyCode::Char(':') => {
                        self.vim_mode = VimSubMode::Command;
                        self.vim_command_buffer.clear();
                    }
                    KeyCode::Char('h') | KeyCode::Left => {
                        self.documents[self.active_tab].cursor.move_left();
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.cursor.move_down(&doc.buffer);
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.cursor.move_up();
                    }
                    KeyCode::Char('l') | KeyCode::Right => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.cursor.move_right(&doc.buffer);
                    }
                    KeyCode::Char('w') => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.cursor.move_word_forward(&doc.buffer);
                    }
                    KeyCode::Char('b') => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.cursor.move_word_backward(&doc.buffer);
                    }
                    KeyCode::Char('0') | KeyCode::Home => {
                        self.documents[self.active_tab].cursor.col = 0;
                    }
                    KeyCode::Char('$') | KeyCode::End => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.cursor.move_to_end_of_line(&doc.buffer);
                    }
                    KeyCode::Char('G') => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.cursor.move_to_bottom(&doc.buffer);
                    }
                    KeyCode::Char('g') => {
                        // gg = go to top (simplified: single g goes to top)
                        self.documents[self.active_tab].cursor.row = 0;
                        self.documents[self.active_tab].cursor.col = 0;
                    }
                    KeyCode::Char('x') => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.delete_char();
                    }
                    KeyCode::Char('d') => {
                        // Simplified: dd deletes line
                        let doc = &mut self.documents[self.active_tab];
                        doc.delete_line();
                    }
                    KeyCode::Char('/') => {
                        self.searching = true;
                        self.search_query.clear();
                    }
                    KeyCode::Char('u') => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.undo();
                    }
                    _ => {}
                }
            }
            VimSubMode::Insert => {
                match key.code {
                    KeyCode::Esc => { self.vim_mode = VimSubMode::Normal; }
                    KeyCode::Char(c) => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.insert_char(c);
                    }
                    KeyCode::Enter => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.insert_newline();
                    }
                    KeyCode::Backspace => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.backspace();
                    }
                    KeyCode::Delete => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.delete_char();
                    }
                    KeyCode::Left => {
                        self.documents[self.active_tab].cursor.move_left();
                    }
                    KeyCode::Right => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.cursor.move_right(&doc.buffer);
                    }
                    KeyCode::Up => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.cursor.move_up();
                    }
                    KeyCode::Down => {
                        let doc = &mut self.documents[self.active_tab];
                        doc.cursor.move_down(&doc.buffer);
                    }
                    KeyCode::Tab => {
                        let doc = &mut self.documents[self.active_tab];
                        for _ in 0..4 {
                            doc.insert_char(' ');
                        }
                    }
                    _ => {}
                }
            }
            VimSubMode::Command => {
                match key.code {
                    KeyCode::Esc => {
                        self.vim_mode = VimSubMode::Normal;
                        self.vim_command_buffer.clear();
                    }
                    KeyCode::Enter => {
                        let cmd = self.vim_command_buffer.clone();
                        self.vim_mode = VimSubMode::Normal;
                        self.execute_vim_command(&cmd);
                        self.vim_command_buffer.clear();
                    }
                    KeyCode::Char(c) => {
                        self.vim_command_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        self.vim_command_buffer.pop();
                        if self.vim_command_buffer.is_empty() {
                            self.vim_mode = VimSubMode::Normal;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn execute_vim_command(&mut self, cmd: &str) {
        match cmd.trim() {
            "w" => self.save_current(),
            "q" => {
                if self.documents.is_empty() || !self.documents[self.active_tab].modified {
                    self.close_current_tab();
                    if self.documents.is_empty() {
                        self.should_quit = true;
                    }
                } else {
                    self.set_status("Unsaved changes! Use :q! to force quit or :wq to save and quit");
                }
            }
            "q!" => {
                self.close_current_tab();
                if self.documents.is_empty() {
                    self.should_quit = true;
                }
            }
            "wq" | "x" => {
                self.save_current();
                self.close_current_tab();
                if self.documents.is_empty() {
                    self.should_quit = true;
                }
            }
            _ => {
                if let Some(filename) = cmd.strip_prefix("e ") {
                    self.open_file(filename.trim());
                } else if let Some(filename) = cmd.strip_prefix("w ") {
                    if !self.documents.is_empty() {
                        self.documents[self.active_tab].file_path = Some(filename.trim().to_string());
                        self.save_current();
                    }
                } else {
                    self.set_status(&format!("Unknown command: :{}", cmd));
                }
            }
        }
    }

    fn handle_nano_input(&mut self, key: KeyEvent) {
        // Forward to Lua plugins first
        let key_str = format!("{:?}", key.code);
        self.last_lua_key = key_str.clone();
        let ptr = self as *mut _;
        if let Ok(true) = self.plugin_manager.run_keypress_hook(&key_str, ptr) {
            return;
        }

        if self.documents.is_empty() { return; }
        let doc = &mut self.documents[self.active_tab];

        match key.code {
            KeyCode::Char(c) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL) {
                    doc.insert_char(c);
                }
            }
            KeyCode::Enter => doc.insert_newline(),
            KeyCode::Backspace => doc.backspace(),
            KeyCode::Delete => doc.delete_char(),
            KeyCode::Left => doc.cursor.move_left(),
            KeyCode::Right => doc.cursor.move_right(&doc.buffer),
            KeyCode::Up => doc.cursor.move_up(),
            KeyCode::Down => doc.cursor.move_down(&doc.buffer),
            KeyCode::Home => doc.cursor.col = 0,
            KeyCode::End => doc.cursor.move_to_end_of_line(&doc.buffer),
            KeyCode::PageUp => {
                for _ in 0..20 { doc.cursor.move_up(); }
            }
            KeyCode::PageDown => {
                for _ in 0..20 { doc.cursor.move_down(&doc.buffer); }
            }
            KeyCode::Tab => {
                for _ in 0..4 { doc.insert_char(' '); }
            }
            _ => {}
        }
    }

    fn handle_aether_input(&mut self, key: KeyEvent) {
        // Forward to Lua plugins first
        let key_str = format!("{:?}", key.code);
        self.last_lua_key = key_str.clone();
        let ptr = self as *mut _;
        if let Ok(true) = self.plugin_manager.run_keypress_hook(&key_str, ptr) {
            return;
        }

        // Aether mode: Smart hybrid mode
        // - Direct text input like Nano (no modal switching needed)
        // - But with smart shortcuts:
        //   Alt+hjkl for fast navigation (vim-style without modes)
        //   Alt+d delete line, Alt+y copy line, Alt+p paste
        //   Smart auto-indent, bracket matching
        if self.documents.is_empty() { return; }

        if key.modifiers.contains(KeyModifiers::ALT) {
            let doc = &mut self.documents[self.active_tab];
            match key.code {
                KeyCode::Char('h') => doc.cursor.move_left(),
                KeyCode::Char('j') => doc.cursor.move_down(&doc.buffer),
                KeyCode::Char('k') => doc.cursor.move_up(),
                KeyCode::Char('l') => doc.cursor.move_right(&doc.buffer),
                KeyCode::Char('w') => doc.cursor.move_word_forward(&doc.buffer),
                KeyCode::Char('b') => doc.cursor.move_word_backward(&doc.buffer),
                KeyCode::Char('d') => doc.delete_line(),
                KeyCode::Char('u') => doc.undo(),
                KeyCode::Char('0') => doc.cursor.col = 0,
                KeyCode::Char('$') => doc.cursor.move_to_end_of_line(&doc.buffer),
                _ => {}
            }
            return;
        }

        let doc = &mut self.documents[self.active_tab];
        match key.code {
            KeyCode::Char(c) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL) {
                    doc.insert_char(c);
                    // Auto-pair brackets in Aether mode
                    match c {
                        '(' => doc.insert_char_no_move(')'),
                        '[' => doc.insert_char_no_move(']'),
                        '{' => doc.insert_char_no_move('}'),
                        '"' => doc.insert_char_no_move('"'),
                        '\'' => doc.insert_char_no_move('\''),
                        _ => {}
                    }
                }
            }
            KeyCode::Enter => {
                // Smart auto-indent
                let indent = doc.get_current_indent();
                doc.insert_newline();
                for c in indent.chars() {
                    doc.insert_char(c);
                }
            }
            KeyCode::Backspace => doc.backspace(),
            KeyCode::Delete => doc.delete_char(),
            KeyCode::Left => doc.cursor.move_left(),
            KeyCode::Right => doc.cursor.move_right(&doc.buffer),
            KeyCode::Up => doc.cursor.move_up(),
            KeyCode::Down => doc.cursor.move_down(&doc.buffer),
            KeyCode::Home => doc.cursor.col = 0,
            KeyCode::End => doc.cursor.move_to_end_of_line(&doc.buffer),
            KeyCode::PageUp => { for _ in 0..20 { doc.cursor.move_up(); } }
            KeyCode::PageDown => { for _ in 0..20 { doc.cursor.move_down(&doc.buffer); } }
            KeyCode::Tab => {
                for _ in 0..4 { doc.insert_char(' '); }
            }
            _ => {}
        }
    }

    fn handle_emacs_input(&mut self, key: KeyEvent) {
        // Forward to Lua plugins first
        let key_str = format!("{:?}", key.code);
        self.last_lua_key = key_str.clone();
        let ptr = self as *mut _;
        if let Ok(true) = self.plugin_manager.run_keypress_hook(&key_str, ptr) {
            return;
        }

        if self.documents.is_empty() { return; }
        let doc = &mut self.documents[self.active_tab];

        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('p') => doc.cursor.move_up(),
                KeyCode::Char('n') => doc.cursor.move_down(&doc.buffer),
                KeyCode::Char('f') => doc.cursor.move_right(&doc.buffer),
                KeyCode::Char('b') => doc.cursor.move_left(),
                KeyCode::Char('a') => doc.cursor.col = 0,
                KeyCode::Char('e') => doc.cursor.move_to_end_of_line(&doc.buffer),
                KeyCode::Char('k') => doc.delete_line(), // Emacs "kill-line"
                KeyCode::Char('y') => {
                    // Yank is paste
                    // For now, simpler: we don't have a kill ring yet, but doc has undo/redo.
                }
                KeyCode::Char('d') => doc.delete_char(),
                KeyCode::Char('h') => doc.backspace(),
                KeyCode::Char('s') => self.save_current(),
                KeyCode::Char('/') => doc.undo(),
                _ => {}
            }
            return;
        }

        if key.modifiers.contains(KeyModifiers::ALT) {
            match key.code {
                KeyCode::Char('f') => doc.cursor.move_word_forward(&doc.buffer),
                KeyCode::Char('b') => doc.cursor.move_word_backward(&doc.buffer),
                KeyCode::Char('d') => {
                    // Kill word - simplified to delete char for now
                    doc.delete_char();
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Char(c) => doc.insert_char(c),
            KeyCode::Enter => doc.insert_newline(),
            KeyCode::Backspace => doc.backspace(),
            KeyCode::Delete => doc.delete_char(),
            KeyCode::Tab => {
                for _ in 0..4 { doc.insert_char(' '); }
            }
            KeyCode::Up => doc.cursor.move_up(),
            KeyCode::Down => doc.cursor.move_down(&doc.buffer),
            KeyCode::Left => doc.cursor.move_left(),
            KeyCode::Right => doc.cursor.move_right(&doc.buffer),
            _ => {}
        }
    }

    pub fn handle_command_palette_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.screen = AppScreen::Editor;
            }
            KeyCode::Up => {
                if self.command_palette.selected > 0 {
                    self.command_palette.selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.command_palette.selected + 1 < self.command_palette.filtered.len() {
                    self.command_palette.selected += 1;
                }
            }
            KeyCode::Enter => {
                if let Some(&idx) = self.command_palette.filtered.get(self.command_palette.selected) {
                    self.execute_palette_command(idx);
                }
                self.screen = AppScreen::Editor;
            }
            KeyCode::Char(c) => {
                self.command_palette.query.push(c);
                self.command_palette.update_filter();
            }
            KeyCode::Backspace => {
                self.command_palette.query.pop();
                self.command_palette.update_filter();
            }
            _ => {}
        }
    }

    fn execute_palette_command(&mut self, idx: usize) {
        match idx {
            0 => self.open_file_picker(),
            1 => self.save_current(),
            2 => { 
                self.save_prompt_active = true;
                self.save_prompt_query.clear();
            }
            3 => self.close_current_tab(),
            4 => self.new_file(),
            5 => { self.show_file_tree = !self.show_file_tree; }
            6 => self.cycle_theme(),
            7 => {
                self.edit_mode = EditMode::Vim;
                self.vim_mode = VimSubMode::Normal;
                self.config.edit_mode = "vim".to_string();
                let _ = self.config.save();
                self.set_status("Switched to Vim mode");
            }
            8 => {
                self.edit_mode = EditMode::Nano;
                self.config.edit_mode = "nano".to_string();
                let _ = self.config.save();
                self.set_status("Switched to Nano mode");
            }
            9 => {
                self.edit_mode = EditMode::Aether;
                self.config.edit_mode = "aether".to_string();
                let _ = self.config.save();
                self.set_status("Switched to Aether mode");
            }
            10 => { self.show_line_numbers = !self.show_line_numbers; }
            11 => { /* Go to line - simplified */ }
            12 => { self.searching = true; self.search_query.clear(); }
            13 => { self.word_wrap = !self.word_wrap; }
            14 => {
                if !self.documents.is_empty() {
                    let line_content = {
                        let doc = &self.documents[self.active_tab];
                        doc.buffer.get_line(doc.cursor.row).to_string()
                    };
                    self.show_ai_sidebar = true;
                    self.focus = AppFocus::AiPrompt;
                    self.send_ai_message(&format!("Complete this code:\n```\n{}\n```", line_content));
                }
            }
            15 => {
                if !self.documents.is_empty() {
                    let line_content = {
                        let doc = &self.documents[self.active_tab];
                        doc.buffer.get_line(doc.cursor.row).to_string()
                    };
                    self.show_ai_sidebar = true;
                    self.focus = AppFocus::AiPrompt;
                    self.send_ai_message(&format!("Explain this code:\n```\n{}\n```", line_content));
                }
            }
            16 => {
                self.show_ai_sidebar = true;
                self.focus = AppFocus::AiPrompt;
            }
            17 => {
                self.show_ai_sidebar = !self.show_ai_sidebar;
            }
            18 => {
                self.screen = AppScreen::GitStatus;
            }
            19 => {
                let status = std::process::Command::new("git")
                    .arg("status")
                    .arg("--short")
                    .output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                    .unwrap_or_else(|_| "Git error".to_string());
                self.set_status(&format!("Git: {}", status.replace('\n', " ")));
            }
            20 => {
                let _ = std::process::Command::new("git").arg("add").arg(".").status();
                self.set_status("Git: Added all changes");
            }
            21 => {
                let _ = std::process::Command::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg("Updates from Aether Editor")
                    .status();
                self.set_status("Git: Committed changes");
            }
            22 => {
                let _ = std::process::Command::new("git").arg("push").status();
                self.set_status("Git: Pushed commits to remote");
            }
            23 => {
                // Uses GitHub CLI to authenticate through web browser
                let _ = std::process::Command::new("gh")
                    .arg("auth")
                    .arg("login")
                    .arg("-w")
                    .spawn();
                self.set_status("GitHub: Login process initiated");
            }
            24 => {
                self.show_lua_info = !self.show_lua_info;
                self.config.show_lua_info = self.show_lua_info;
                let _ = self.config.save();
                self.set_status(&format!("Lua info display: {}", if self.show_lua_info { "Enabled" } else { "Disabled" }));
            }
            25 => {
                self.show_tab_switch_hint = !self.show_tab_switch_hint;
                self.config.show_tab_switch_hint = self.show_tab_switch_hint;
                let _ = self.config.save();
                self.set_status(&format!("Tab switch hints: {}", if self.show_tab_switch_hint { "Enabled" } else { "Disabled" }));
            }
            26 => { self.screen = AppScreen::Controls; }
            27 => { self.should_quit = true; }
            _ => {}
        }
    }

    pub fn open_file_picker(&mut self) {
        self.file_picker_state = FilePickerState::new();
        self.file_picker_active = true;
        self.screen = AppScreen::Editor;
    }

    pub fn handle_file_picker_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.file_picker_active = false;
                if self.documents.is_empty() {
                    self.screen = AppScreen::Welcome;
                }
            }
            KeyCode::Up => {
                if self.file_picker_state.selected > 0 {
                    self.file_picker_state.selected -= 1;
                    // Scroll up if needed
                    if self.file_picker_state.selected < self.file_picker_state.scroll {
                        self.file_picker_state.scroll = self.file_picker_state.selected;
                    }
                }
            }
            KeyCode::Down => {
                if self.file_picker_state.selected + 1 < self.file_picker_state.filtered_entries.len() {
                    self.file_picker_state.selected += 1;
                }
            }
            KeyCode::Enter => {
                if let Some(path) = self.file_picker_state.enter_selected() {
                    self.file_picker_active = false;
                    let new_tab = key.modifiers.contains(KeyModifiers::CONTROL);
                    if !new_tab && !self.documents.is_empty() {
                        // Open in current tab
                        match crate::editor::document::Document::open(&path) {
                            Ok(doc) => {
                                let name = doc.file_name().to_string();
                                self.documents[self.active_tab] = doc;
                                self.set_status(&format!("Opened {}", name));
                                let path_str = path.to_string();
                                self.config.recent_files.retain(|f| f != &path_str);
                                self.config.recent_files.insert(0, path_str);
                                if self.config.recent_files.len() > 10 {
                                    self.config.recent_files.truncate(10);
                                }
                                let _ = self.config.save();
                            }
                            Err(e) => {
                                self.set_status(&format!("Error opening file: {}", e));
                            }
                        }
                    } else {
                        // Open in new tab
                        self.open_file(&path);
                    }
                }
            }
            KeyCode::Backspace => {
                if self.file_picker_state.filter_query.is_empty() {
                    self.file_picker_state.navigate_up();
                } else {
                    self.file_picker_state.filter_query.pop();
                    self.file_picker_state.update_filter();
                }
            }
            KeyCode::Char(c) => {
                self.file_picker_state.filter_query.push(c);
                self.file_picker_state.update_filter();
            }
            _ => {}
        }
    }

    pub fn refresh_git_diff(&mut self) {
        let file = if self.git_focus == 0 {
            self.git_unstaged.get(self.git_selected).map(|s| &s[3..])
        } else {
            self.git_staged.get(self.git_selected).map(|s| &s[3..])
        };

        self.git_diff_content.clear();
        if let Some(file) = file {
            let mut cmd = std::process::Command::new("git");
            cmd.arg("diff");
            if self.git_focus == 1 {
                cmd.arg("--cached");
            }
            cmd.arg(file);

            if let Ok(output) = cmd.output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                self.git_diff_content = stdout.lines().map(|l| l.to_string()).collect();
            }
        }
    }

    pub fn execute_git_commit(&mut self) {
        if self.git_commit_message.trim().is_empty() {
            self.set_status("Commit message cannot be empty");
            return;
        }

        let output = std::process::Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(&self.git_commit_message)
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                self.set_status("Git: Committed changes");
                self.git_commit_message.clear();
                self.refresh_git_status();
                self.screen = AppScreen::GitStatus;
            } else {
                let err = String::from_utf8_lossy(&output.stderr);
                self.set_status(&format!("Git Error: {}", err));
            }
        }
    }

    pub fn refresh_git_status(&mut self) {
        let output = std::process::Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .output();

        self.git_unstaged.clear();
        self.git_staged.clear();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.len() < 4 { continue; }
                let staged_code = &line[0..1];
                let unstaged_code = &line[1..2];
                let file = &line[3..];

                if staged_code != " " && staged_code != "?" {
                    self.git_staged.push(format!("{} {}", staged_code, file));
                }
                if unstaged_code != " " || staged_code == "?" {
                    let code = if staged_code == "?" { "??" } else { unstaged_code };
                    self.git_unstaged.push(format!("{} {}", code, file));
                }
            }
        }
    }
fn build_file_tree(path: &str, depth: usize, max_depth: usize) -> Vec<FileTreeEntry> {
    let mut entries = Vec::new();
    if depth > max_depth { return entries; }

    let Ok(read_dir) = std::fs::read_dir(path) else { return entries };
    let mut items: Vec<_> = read_dir
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            !name.starts_with('.') && name != "target" && name != "node_modules"
        })
        .collect();

    items.sort_by(|a, b| {
        let a_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let b_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
        b_dir.cmp(&a_dir).then(a.file_name().cmp(&b.file_name()))
    });

    for item in items {
        let name = item.file_name().to_string_lossy().to_string();
        let item_path = item.path().to_string_lossy().to_string();
        let is_dir = item.file_type().map(|t| t.is_dir()).unwrap_or(false);

        entries.push(FileTreeEntry {
            name: name.clone(),
            path: item_path.clone(),
            is_dir,
            depth,
            expanded: depth == 0,
        });

        if is_dir && depth < max_depth {
            entries.extend(Self::build_file_tree(&item_path, depth + 1, max_depth));
        }
    }

    entries
}
}
