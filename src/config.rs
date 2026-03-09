use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub theme_index: usize,
    pub edit_mode: String,
    pub ai_enabled: bool,
    pub ai_backend: String,
    pub ai_model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_api_key: Option<String>,
    pub auto_update: bool,
    pub mouse_support: bool,
    pub show_line_numbers: bool,
    pub tab_size: usize,
    pub word_wrap: bool,
    pub font_ligatures: bool,
    pub recent_files: Vec<String>,
    pub first_run: bool,
    pub show_lua_info: bool,
    pub show_tab_switch_hint: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: whoami::username(),
            theme_index: 0,
            edit_mode: "aether".to_string(),
            ai_enabled: false,
            ai_backend: "none".to_string(),
            ai_model: "none".to_string(),
            ai_api_key: None,
            auto_update: true,
            mouse_support: true,
            show_line_numbers: true,
            tab_size: 4,
            word_wrap: false,
            font_ligatures: false,
            recent_files: Vec::new(),
            first_run: true,
            show_lua_info: false,
            show_tab_switch_hint: true,
        }
    }
}

impl Config {
    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("aether")
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    pub fn config_exists() -> bool {
        Self::config_path().exists()
    }

    pub fn load_or_default() -> Self {
        if let Ok(content) = std::fs::read_to_string(Self::config_path()) {
            toml::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let dir = Self::config_dir();
        std::fs::create_dir_all(&dir)?;
        let content =
            toml::to_string_pretty(self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        std::fs::write(Self::config_path(), content)
    }
}
