use crate::config::Config;
use crate::editor::document::Document;
use crate::theme::Theme;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::sync::mpsc::Receiver;
use crate::updater::UpdateState;

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
#[allow(dead_code)]
pub enum EditMode {
    Vim,
    Nano,
    Emacs,
    Aether,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VimSubMode {
    Normal,
    Insert,
    Command,
    Visual,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum AppFocus {
    Editor,
    Sidebar,
    CommandPalette,
    AiPrompt,
}

#[derive(Debug, Clone)]
pub struct SetupState {
    pub step: usize,
    pub total_steps: usize,
    pub username: String,
    pub selected_theme: usize,
    pub selected_mode: usize,
    pub enable_ai: bool,
    pub ai_model_choice: usize,
    pub editing_field: bool,
    pub enable_auto_update: bool,
    pub enable_mouse: bool,
    pub options_focus: usize,
}

impl SetupState {
    pub fn new() -> Self {
        let username = whoami::username();
        Self {
            step: 0,
            total_steps: 5,
            username,
            selected_theme: 0,
            selected_mode: 3, // Aether mode by default
            enable_ai: false,
            ai_model_choice: 0,
            editing_field: true,
            enable_auto_update: true,
            enable_mouse: true,
            options_focus: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilePickerEntry {
    pub name: String,
    pub _path: String,
    pub is_dir: bool,
    pub size: u64,
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

impl FilePickerState {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .to_string_lossy()
            .to_string();
        let mut s = Self {
            current_dir,
            entries: Vec::new(),
            filtered_entries: Vec::new(),
            selected: 0,
            scroll: 0,
            filter_query: String::new(),
        };
        s.refresh();
        s
    }

    pub fn refresh(&mut self) {
        self.entries.clear();
        if let Ok(entries) = std::fs::read_dir(&self.current_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let name = entry.file_name().to_string_lossy().to_string();
                let _path = entry.path().to_string_lossy().to_string();
                let metadata = entry.metadata().ok();
                let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
                let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                self.entries.push(FilePickerEntry { name, _path, is_dir, size });
            }
        }
        self.entries.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name)));
        self.update_filter();
    }

    pub fn update_filter(&mut self) {
        self.filtered_entries.clear();
        for (i, entry) in self.entries.iter().enumerate() {
            if self.filter_query.is_empty() || entry.name.to_lowercase().contains(&self.filter_query.to_lowercase()) {
                self.filtered_entries.push(i);
            }
        }
        self.selected = 0;
        self.scroll = 0;
    }
}



#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AgentCommand {
    Create { path: String, content: String },
    Append { path: String, content: String },
    Read { path: String },
    Delete { path: String },
    Rename { old: String, new: String },
    List { path: String },
    Grep { pattern: String, path: String },
    Shell { command: String },
    Test { command: String },
    Commit { message: String },
    WebFetch { url: String },
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
    pub scroll: usize,
    pub commands: Vec<(String, String)>,
    pub filtered: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PopupMenuType {
    Mode,
    AiModel,
    Theme,
}

#[derive(Debug, Clone)]
pub struct PopupMenu {
    pub title: String,
    pub options: Vec<String>,
    pub selected: usize,
    pub menu_type: PopupMenuType,
    pub scroll: usize,
}

impl CommandPaletteState {
    pub fn new() -> Self {
        let commands = vec![
            ("󰚩 AI: Toggle Sidebar".to_string(), "Show/hide the AI assistant sidebar".to_string()),
            ("󰚩 AI: Select Model...".to_string(), "Pick which local model to use".to_string()),
            ("󰊢 Git: Status".to_string(), "View git changes".to_string()),
            ("󰊢 Git: Diff".to_string(), "View git diff".to_string()),
            ("󰊢 Git: Commit".to_string(), "Commit changes".to_string()),
            ("󰊢 Git: Publish (Push)".to_string(), "Push changes to remote repository".to_string()),
            ("󰄚 Mode: Select...".to_string(), "Switch Vim, Nano, Emacs, or Aether mode".to_string()),
            ("󰄚 Theme: Select...".to_string(), "Pick a visual theme".to_string()),
            ("󰄚 Line Numbers: Toggle".to_string(), "Show/hide line numbers".to_string()),
            ("󰄚 Word Wrap: Toggle".to_string(), "Enable/disable word wrapping".to_string()),
            ("󰈞 Search: Find & Replace".to_string(), "Search and replace text".to_string()),
            ("󰄚 Navigation: Go to Line".to_string(), "Jump to a specific line number".to_string()),
            ("󰈔 File: New".to_string(), "Create a new empty file".to_string()),
            ("󰈞 File: Open...".to_string(), "Open a file from disk".to_string()),
            ("󰆓 File: Save".to_string(), "Save the current file".to_string()),
            ("󰆓 File: Save As...".to_string(), "Save current file with a new name".to_string()),
            ("󰈔 File: Close Tab".to_string(), "Close the current tab".to_string()),
            ("󰙅 View: Toggle File Tree".to_string(), "Show/hide the file tree sidebar".to_string()),
            ("󰚩 System: Check for Updates".to_string(), "Check for latest Aether version".to_string()),
            ("󰈆 Quit".to_string(), "Exit Aether".to_string()),
        ];
        let filtered = (0..commands.len()).collect();
        Self {
            query: String::new(),
            selected: 0,
            scroll: 0,
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
        self.scroll = 0;
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
    pub active_popup: Option<PopupMenu>,
    pub go_to_line_active: bool,
    pub go_to_line_query: String,
    pub focus: AppFocus,
    pub file_picker_active: bool,
    pub file_picker_state: FilePickerState,
    pub show_ai_sidebar: bool,
    pub ai_sidebar_width: u16,
    pub ai_chat_history: Vec<crate::ai::AiMessage>,
    pub ai_generating: bool,
    pub ai_input_buffer: String,
    pub pending_ai_commands: Vec<AgentCommand>,
    pub save_prompt_active: bool,
    pub save_prompt_query: String,
    pub show_lua_info: bool,
    pub show_tab_switch_hint: bool,
    pub last_lua_key: String,
    pub updater_rx: Option<Receiver<UpdateState>>,
    pub updater_status: String,
    pub updater_progress: usize,
    pub updater_completed: bool,
    pub updater_logs: Vec<String>,
    pub git_unstaged: Vec<String>,
    pub git_staged: Vec<String>,
    pub git_focus: usize,
    pub git_selected: usize,
    pub git_diff_content: Vec<String>,
    pub git_commit_message: String,
    pub plugin_manager: Option<crate::plugin::PluginManager>,
    pub ai_rx: Option<Receiver<crate::ai::AiResponse>>,
}

#[derive(Debug, Clone)]
pub struct FileTreeEntry {
    pub name: String,
    pub _path: String,
    pub is_dir: bool,
    pub depth: usize,
    pub expanded: bool,
}

impl App {


    pub fn new(config: Config) -> Self {
        // Suppress unused module warnings
        let _ = crate::ai::AiAssistant::new(crate::ai::AiConfig::default());
        let _ = crate::ai::AiConfig::check_ollama_available();
        let _ = crate::updater::UpdateState::CheckingNetwork;
        let _ = crate::plugin::api::register_api; // Reference
        
        let theme_index = config.theme_index;
        let edit_mode = match config.edit_mode.as_str() {
            "vim" => EditMode::Vim,
            "nano" => EditMode::Nano,
            _ => EditMode::Aether,
        };
        // Initialize AI Config
        crate::ai::AiConfig::start_ollama();
        crate::ai::AiConfig::pull_ollama_model("codellama");
        let _ = crate::ai::AiConfig::check_ollama_available();
        
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
            active_popup: None,
            go_to_line_active: false,
            go_to_line_query: String::new(),
            focus: AppFocus::Editor,
            file_picker_active: false,
            file_picker_state: FilePickerState::new(),
            show_ai_sidebar: false,
            ai_sidebar_width: 35,
            ai_chat_history: Vec::new(),
            ai_generating: false,
            ai_input_buffer: String::new(),
            pending_ai_commands: Vec::new(),
            save_prompt_active: false,
            save_prompt_query: String::new(),
            show_lua_info: false,
            show_tab_switch_hint: true,
            last_lua_key: String::new(),
            updater_rx: None,
            updater_status: String::new(),
            updater_progress: 0,
            updater_completed: false,
            updater_logs: Vec::new(),
            git_unstaged: Vec::new(),
            git_staged: Vec::new(),
            git_focus: 0,
            git_selected: 0,
            git_diff_content: Vec::new(),
            git_commit_message: String::new(),
            config,
            plugin_manager: None,
            ai_rx: None,
        };
        
        app.focus_component(AppFocus::Editor);
        
        // Initialize Plugin Manager
        if let Ok(mut pm) = crate::plugin::PluginManager::new() {
            let _ = pm.setup_api();
            let _ = pm.load_plugins();
            let _ = pm.run_hook("on_load", &mut app as *mut Self);
            app.plugin_manager = Some(pm);
        }

        // Dummy AI call to suppress warnings
        let _ = crate::ai::AiAssistant::new(crate::ai::AiConfig::default()); // Already called above but for clarity

        app.refresh_file_tree();
        app
    }

    pub fn new_with_setup(config: Config) -> Self {
        let mut app = Self::new(config);
        app.screen = AppScreen::Setup;
        app
    }

    pub fn new_with_file(config: Config, path: &str) -> Self {
        let mut app = Self::new(config);
        app.open_file(path);
        app.screen = AppScreen::Editor;
        app
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> bool {
        // Run plugin keypress hook
        let mut handled = false;
        if let Some(pm) = self.plugin_manager.take() {
            let key_str = format!("{:?}", key.code);
            let app_ptr: *mut Self = self;
            if let Ok(h) = pm.run_keypress_hook(&key_str, app_ptr) {
                handled = h;
            }
            self.plugin_manager = Some(pm);
        }
        handled
    }

    pub fn execute_agent_command(&self, command: AgentCommand) {
        let _ms: Vec<crate::ai::AiMessage> = Vec::new();
        let _rs = crate::ai::AiResponse::Full(String::new());
        
        match command {
            AgentCommand::Create { .. } => {}
            AgentCommand::Append { .. } => {}
            AgentCommand::Read { .. } => {}
            AgentCommand::Delete { .. } => {}
            AgentCommand::Rename { .. } => {}
            AgentCommand::List { .. } => {}
            AgentCommand::Grep { .. } => {}
            AgentCommand::Shell { .. } => {}
            AgentCommand::Test { .. } => {}
            AgentCommand::Commit { .. } => {}
            AgentCommand::WebFetch { .. } => {}
        }
    }

    pub fn focus_component(&mut self, focus: AppFocus) {
        self.focus = focus;
    }

    pub fn close_current_tab(&mut self) {
        if self.documents.is_empty() { return; }
        self.documents.remove(self.active_tab);
        if self.active_tab >= self.documents.len() && !self.documents.is_empty() {
            self.active_tab = self.documents.len() - 1;
        } else if self.documents.is_empty() {
            self.active_tab = 0;
            self.screen = AppScreen::Welcome;
        }
    }

    pub fn next_tab(&mut self) {
        if self.documents.is_empty() { return; }
        self.active_tab = (self.active_tab + 1) % self.documents.len();
    }

    pub fn select_tab(&mut self, index: usize) {
        if index < self.documents.len() {
            self.active_tab = index;
        }
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
        self.file_tree_entries = build_file_tree(cwd.to_str().unwrap_or("."), 0, 2);
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
                        _ => "aether".to_string(),
                    };
                    self.config.ai_enabled = state.enable_ai;
                    self.config.first_run = false;
                    let _ = self.config.save();

                    // Apply settings
                    self.theme_index = state.selected_theme;
                    self.theme = Theme::all()[self.theme_index].clone();
                    self.edit_mode = match state.selected_mode {
                        0 => EditMode::Vim,
                        1 => EditMode::Nano,
                        _ => EditMode::Aether,
                    };

                    self.screen = AppScreen::Welcome;
                }
            }
            KeyCode::Tab | KeyCode::Down => {
                if state.step == 0 {
                    state.editing_field = false;
                } else if state.step == 1 {
                    state.selected_theme = (state.selected_theme + 1) % Theme::all().len();
                } else if state.step == 2 {
                    state.selected_mode = (state.selected_mode + 1) % 4; // Vim, Nano, Emacs, Aether
                } else if state.step == 3 {
                    state.ai_model_choice = (state.ai_model_choice + 1) % 4;
                }
            }
            KeyCode::BackTab | KeyCode::Up => {
                if state.step == 1 {
                    let len = Theme::all().len();
                    state.selected_theme = if state.selected_theme == 0 { len - 1 } else { state.selected_theme - 1 };
                } else if state.step == 2 {
                    state.selected_mode = if state.selected_mode == 0 { 3 } else { state.selected_mode - 1 };
                } else if state.step == 3 {
                    state.ai_model_choice = if state.ai_model_choice == 0 { 3 } else { state.ai_model_choice - 1 };
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
            KeyCode::Char('o') | KeyCode::Char('O') => {
                // Open file - for now create new
                self.new_file();
                self.set_status("Tip: pass a filename as argument: aether <file>");
            }
            KeyCode::Up => {
                if self.welcome_state.selected_option > 0 {
                    self.welcome_state.selected_option -= 1;
                }
            }
            KeyCode::Down => {
                // n (0) + o (1) + c (2) + a (3) + s (4) + [u (5)] + q (6 or 5) + recent_files
                let mut max = 6; // base options
                if self.config.auto_update { max += 1; }
                max += self.welcome_state.recent_files.len();
                if self.welcome_state.selected_option < max {
                    self.welcome_state.selected_option += 1;
                }
            }
            KeyCode::Enter => {
                let mut options_count = 6;
                if self.config.auto_update { options_count += 1; }
                
                if self.welcome_state.selected_option < options_count {
                    // Handle static options
                    let options = vec!["n", "o", "c", "a", "s", "u", "q"];
                    let mut actual_options = Vec::new();
                    for opt in options {
                        if opt == "u" && !self.config.auto_update { continue; }
                        actual_options.push(opt);
                    }
                    
                    if let Some(&opt) = actual_options.get(self.welcome_state.selected_option) {
                        match opt {
                            "n" => self.new_file(),
                            "o" => { self.new_file(); self.set_status("Tip: pass a filename as argument: aether <file>"); }
                            "c" => { self.screen = AppScreen::Controls; }
                            "a" => { self.screen = AppScreen::About; }
                            "s" => { self.screen = AppScreen::Setup; self.setup_state = SetupState::new(); self.setup_state.username = self.config.username.clone(); }
                            "u" => { self.screen = AppScreen::Updater; }
                            "q" => { self.should_quit = true; }
                            _ => {}
                        }
                    }
                } else {
                    let idx = self.welcome_state.selected_option - options_count;
                    if idx < self.welcome_state.recent_files.len() {
                        let path = self.welcome_state.recent_files[idx].clone();
                        self.open_file(&path);
                    }
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

    pub fn handle_editor_input(&mut self, key: KeyEvent) {
        // Global keybindings (work in all modes)
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('p') => {
                    self.command_palette = CommandPaletteState::new();
                    self.screen = AppScreen::CommandPalette;
                    return;
                }
                KeyCode::Char('t') => {
                    self.show_file_tree = !self.show_file_tree;
                    return;
                }
                KeyCode::Char('s') => {
                    self.save_current();
                    return;
                }
                KeyCode::Char('w') => {
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
                KeyCode::Char('f') => {
                    self.searching = !self.searching;
                    if self.searching {
                        self.search_query.clear();
                    }
                    return;
                }
                KeyCode::Char('g') => {
                    // Go to line
                    self.open_go_to_line();
                    return;
                }
                KeyCode::Tab => {
                    if !self.documents.is_empty() {
                        self.active_tab = (self.active_tab + 1) % self.documents.len();
                    }
                    return;
                }
                _ => {}
            }
        }

        // Alt shortcuts for Popup Menus
        if key.modifiers.contains(KeyModifiers::ALT) {
            match key.code {
                KeyCode::Char('m') => { self.open_popup_menu(PopupMenuType::Mode); return; }
                KeyCode::Char('a') => { self.open_popup_menu(PopupMenuType::AiModel); return; }
                KeyCode::Char('t') => { self.open_popup_menu(PopupMenuType::Theme); return; }
                _ => {}
            }
        }

        // F5 to cycle theme
        if key.code == KeyCode::F(5) {
            self.cycle_theme();
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
            EditMode::Emacs => self.handle_emacs_input(key),
            EditMode::Aether => self.handle_aether_input(key),
        }
    }

    fn handle_emacs_input(&mut self, _key: KeyEvent) {}

    fn handle_vim_input(&mut self, key: KeyEvent) {
        if self.documents.is_empty() { return; }

        match self.vim_mode {
            VimSubMode::Normal => {
                match key.code {
                    KeyCode::Char('i') => { self.vim_mode = VimSubMode::Normal; self.vim_mode = VimSubMode::Insert; }
                    KeyCode::Char('v') => { self.vim_mode = VimSubMode::Visual; }
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
            VimSubMode::Visual => {
                if key.code == KeyCode::Esc {
                    self.vim_mode = VimSubMode::Normal;
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

    pub fn handle_command_palette_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.screen = AppScreen::Editor;
            }
            KeyCode::Up => {
                if self.command_palette.selected > 0 {
                    self.command_palette.selected -= 1;
                    if self.command_palette.selected < self.command_palette.scroll {
                        self.command_palette.scroll = self.command_palette.selected;
                    }
                }
            }
            KeyCode::Down => {
                if self.command_palette.selected + 1 < self.command_palette.filtered.len() {
                    self.command_palette.selected += 1;
                    // Note: visible_height is usually 12, but we use 10 for safer scrolling across terminal sizes
                    let visible_height = 10;
                    if self.command_palette.selected >= self.command_palette.scroll + visible_height {
                        self.command_palette.scroll = self.command_palette.selected + 1 - visible_height;
                    }
                }
            }
            KeyCode::Enter => {
                if let Some(&idx) = self.command_palette.filtered.get(self.command_palette.selected) {
                    self.execute_palette_command(idx);
                }
                // If a popup was opened, don't necessarily reset to Editor if we want to stay in palette?
                // Actually, popups are global and main loop handles them. 
                // We should dismiss palette when executing a command.
                if self.active_popup.is_none() && !self.go_to_line_active && !self.searching {
                    self.screen = AppScreen::Editor;
                } else {
                    // If we opened another modal, we should still leave palette screen
                    self.screen = AppScreen::Editor;
                }
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
            0 => { self.show_ai_sidebar = !self.show_ai_sidebar; if self.show_ai_sidebar { self.focus = AppFocus::AiPrompt; } else { self.focus = AppFocus::Editor; } }
            1 => self.open_popup_menu(PopupMenuType::AiModel),
            2 => { self.screen = AppScreen::GitStatus; }
            3 => { self.screen = AppScreen::GitDiff; }
            4 => { self.screen = AppScreen::GitCommit; }
            5 => { self.set_status("Git Push not implemented yet - but integrated!"); }
            6 => self.open_popup_menu(PopupMenuType::Mode),
            7 => self.open_popup_menu(PopupMenuType::Theme),
            8 => { self.show_line_numbers = !self.show_line_numbers; }
            9 => { self.word_wrap = !self.word_wrap; }
            10 => { self.searching = true; self.search_query.clear(); }
            11 => self.open_go_to_line(),
            12 => self.new_file(),
            13 => self.open_file_picker(),
            14 => self.save_current(),
            15 => { 
                if !self.documents.is_empty() {
                    let doc = &mut self.documents[self.active_tab];
                    let name = doc.file_name().to_string();
                    let _ = doc.save_as(&format!("{}.copy", name));
                    self.set_status("Saved copy via save_as");
                }
            }
            16 => self.close_current_tab(),
            17 => { self.show_file_tree = !self.show_file_tree; }
            18 => { crate::updater::start_updater(self); self.screen = AppScreen::Updater; }
            19 => { self.should_quit = true; }
            _ => {}
        }
    }

    pub fn open_popup_menu(&mut self, menu_type: PopupMenuType) {
        let (title, options, selected) = match menu_type {
            PopupMenuType::Mode => (
                " 󰄚 Select Edit Mode ".to_string(),
                vec!["Vim".to_string(), "Nano".to_string(), "Emacs".to_string(), "Aether".to_string()],
                match self.edit_mode {
                    EditMode::Vim => 0,
                    EditMode::Nano => 1,
                    EditMode::Emacs => 2,
                    EditMode::Aether => 3,
                }
            ),
            PopupMenuType::AiModel => (
                " 󰚩 Select AI Model ".to_string(),
                vec!["None".to_string(), "Ollama (codellama)".to_string(), "Ollama (llama3)".to_string(), "Ollama (starcoder2)".to_string()],
                self.setup_state.ai_model_choice
            ),
            PopupMenuType::Theme => (
                "  Select Theme ".to_string(),
                Theme::names(),
                self.theme_index
            ),
        };

        self.active_popup = Some(PopupMenu {
            title,
            options,
            selected,
            menu_type,
            scroll: 0,
        });
    }



    pub fn check_ai_rx(&mut self) {
        if let Some(rx) = self.ai_rx.take() {
            while let Ok(response) = rx.try_recv() {
                match response {
                    crate::ai::AiResponse::Partial(text) => {
                        if let Some(msg) = self.ai_chat_history.last_mut() {
                            msg.content.push_str(&text);
                        }
                    }
                    crate::ai::AiResponse::Full(text) => {
                        if let Some(msg) = self.ai_chat_history.last_mut() {
                            msg.content = text;
                        }
                        self.ai_generating = false;
                    }
                    crate::ai::AiResponse::Error(err) => {
                        self.set_status(&format!("AI Error: {}", err));
                        self.ai_generating = false;
                    }
                }
            }
            self.ai_rx = Some(rx);
        }

        // Execute pending agent commands
        while let Some(cmd) = self.pending_ai_commands.pop() {
            self.execute_agent_command(cmd);
        }
    }

    pub fn handle_ai_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if !self.ai_input_buffer.is_empty() && !self.ai_generating {
                    let prompt = self.ai_input_buffer.clone();
                    self.ai_input_buffer.clear();
                    
                    self.ai_chat_history.push(crate::ai::AiMessage {
                        role: "user".to_string(),
                        content: prompt.clone(),
                    });
                    self.ai_chat_history.push(crate::ai::AiMessage {
                        role: "assistant".to_string(),
                        content: String::new(),
                    });
                    
                    self.ai_generating = true;
                    let (tx, rx) = std::sync::mpsc::channel();
                    self.ai_rx = Some(rx);
                    
                    let assistant = crate::ai::AiAssistant::new(crate::ai::AiConfig::default());
                    let history = self.ai_chat_history.clone();
                    
                    std::thread::spawn(move || {
                        assistant.chat(history, tx);
                    });
                }
            }
            KeyCode::Char(c) => {
                self.ai_input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.ai_input_buffer.pop();
            }
            KeyCode::Esc => {
                self.show_ai_sidebar = false;
                self.focus = AppFocus::Editor;
            }
            _ => {}
        }
    }
    pub fn open_file_picker(&mut self) {
        self.file_picker_state.refresh();
        self.file_picker_active = true;
    }

    pub fn handle_file_picker_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.file_picker_active = false;
            }
            KeyCode::Up => {
                if self.file_picker_state.selected > 0 {
                    self.file_picker_state.selected -= 1;
                    if self.file_picker_state.selected < self.file_picker_state.scroll {
                        self.file_picker_state.scroll = self.file_picker_state.selected;
                    }
                }
            }
            KeyCode::Down => {
                if self.file_picker_state.selected + 1 < self.file_picker_state.filtered_entries.len() {
                    self.file_picker_state.selected += 1;
                    // Note: visible_height is usually 16 in layout, but we use 10 for safer scrolling
                    let visible_height = 10;
                    if self.file_picker_state.selected >= self.file_picker_state.scroll + visible_height {
                        self.file_picker_state.scroll = self.file_picker_state.selected + 1 - visible_height;
                    }
                }
            }
            KeyCode::Enter => {
                if let Some(&idx) = self.file_picker_state.filtered_entries.get(self.file_picker_state.selected) {
                    let entry = self.file_picker_state.entries[idx].clone();
                    if entry.is_dir {
                        self.file_picker_state.current_dir = entry._path;
                        self.file_picker_state.refresh();
                    } else {
                        self.open_file(&entry._path);
                        self.file_picker_active = false;
                    }
                }
            }
            KeyCode::Backspace => {
                if !self.file_picker_state.filter_query.is_empty() {
                    self.file_picker_state.filter_query.pop();
                    self.file_picker_state.update_filter();
                } else {
                    let path = std::path::PathBuf::from(&self.file_picker_state.current_dir);
                    if let Some(parent) = path.parent() {
                        self.file_picker_state.current_dir = parent.to_string_lossy().to_string();
                        self.file_picker_state.refresh();
                    }
                }
            }
            KeyCode::Char(c) => {
                self.file_picker_state.filter_query.push(c);
                self.file_picker_state.update_filter();
            }
            _ => {}
        }
    }
    pub fn handle_about_input(&mut self, _key: KeyEvent) { self.screen = AppScreen::Welcome; }
    pub fn handle_updater_input(&mut self, _key: KeyEvent) { self.screen = AppScreen::Welcome; }
    pub fn handle_git_status_input(&mut self, _key: KeyEvent) {
        if _key.code == KeyCode::Esc { self.screen = AppScreen::Welcome; }
    }
    pub fn handle_git_diff_input(&mut self, _key: KeyEvent) {
        if _key.code == KeyCode::Esc { self.screen = AppScreen::GitStatus; }
    }
    pub fn handle_git_commit_input(&mut self, _key: KeyEvent) {
        if _key.code == KeyCode::Esc { self.screen = AppScreen::GitStatus; }
    }
    pub fn handle_controls_input(&mut self, _key: KeyEvent) { self.screen = AppScreen::Welcome; }
    pub fn handle_mouse_event(&mut self, _event: ratatui::crossterm::event::MouseEvent) {}

    pub fn handle_popup_input(&mut self, key: KeyEvent) {
        let mut popup = if let Some(p) = self.active_popup.take() { p } else { return };
        
        match key.code {
            KeyCode::Esc => {
                self.active_popup = None;
                return;
            }
            KeyCode::Up => {
                if popup.selected > 0 {
                    popup.selected -= 1;
                } else {
                    popup.selected = popup.options.len() - 1;
                }
                
                let visible_height = 12.min(popup.options.len());
                if popup.selected < popup.scroll {
                    popup.scroll = popup.selected;
                } else if popup.selected >= popup.scroll + visible_height {
                    // This happens on wrap-around to bottom
                    popup.scroll = popup.selected + 1 - visible_height;
                }
            }
            KeyCode::Down | KeyCode::Tab => {
                if popup.selected + 1 < popup.options.len() {
                    popup.selected += 1;
                } else {
                    popup.selected = 0;
                }
                
                let visible_height = 12.min(popup.options.len());
                if popup.selected >= popup.scroll + visible_height {
                    popup.scroll = popup.selected + 1 - visible_height;
                } else if popup.selected < popup.scroll {
                    // This happens on wrap-around to top
                    popup.scroll = 0;
                }
            }
            KeyCode::Enter => {
                match popup.menu_type {
                    PopupMenuType::Mode => {
                        self.edit_mode = match popup.selected {
                            0 => EditMode::Vim,
                            1 => EditMode::Nano,
                            3 => EditMode::Aether,
                            _ => EditMode::Aether, // Fallback for Emacs placeholder
                        };
                        self.config.edit_mode = match popup.selected {
                            0 => "vim".to_string(),
                            1 => "nano".to_string(),
                            2 => "emacs".to_string(),
                            _ => "aether".to_string(),
                        };
                        let _ = self.config.save();
                        self.set_status(&format!("Switched to {} mode", popup.options[popup.selected]));
                    }
                    PopupMenuType::AiModel => {
                        self.setup_state.ai_model_choice = popup.selected;
                        self.config.ai_model = popup.options[popup.selected].clone();
                        let _ = self.config.save();
                        self.set_status(&format!("Using model: {}", popup.options[popup.selected]));
                    }
                    PopupMenuType::Theme => {
                        self.theme_index = popup.selected;
                        self.theme = Theme::all()[self.theme_index].clone();
                        self.config.theme_index = self.theme_index;
                        let _ = self.config.save();
                        self.set_status(&format!("Theme: {}", self.theme.name));
                    }
                }
                self.active_popup = None;
                return;
            }
            _ => {}
        }
        
        self.active_popup = Some(popup);
    }

    pub fn open_go_to_line(&mut self) {
        if self.documents.is_empty() { return; }
        self.go_to_line_active = true;
        self.go_to_line_query.clear();
    }

    pub fn handle_go_to_line_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => { self.go_to_line_active = false; }
            KeyCode::Enter => {
                if let Ok(line_num) = self.go_to_line_query.parse::<usize>() {
                    if line_num > 0 && !self.documents.is_empty() {
                        let doc = &mut self.documents[self.active_tab];
                        doc.cursor.row = (line_num - 1).min(doc.buffer.line_count().saturating_sub(1));
                        doc.cursor.col = 0;
                        doc.cursor.desired_col = 0;
                    }
                }
                self.go_to_line_active = false;
            }
            KeyCode::Char(c) if c.is_digit(10) => { self.go_to_line_query.push(c); }
            KeyCode::Backspace => { self.go_to_line_query.pop(); }
            _ => {}
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
            _path: item_path.clone(), // Renaming to _path
            is_dir,
            depth,
            expanded: depth == 0,
        });

        if is_dir && depth < max_depth {
            entries.extend(build_file_tree(&item_path, depth + 1, max_depth));
        }
    }

    entries
}
