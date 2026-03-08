// AI Module — Local AI support for code assistance
//
// Aether supports local AI models for code completion,
// explanation, and refactoring — no internet required.
// This runs entirely on your machine.
//
// Supported backends (planned):
//   - Ollama (llama3, codellama, mistral, etc.)
//   - llama.cpp via GGUF models
//
// The AI can be enabled during setup or later in settings.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub enabled: bool,
    pub backend: AiBackend,
    pub model_name: String,
    pub endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiBackend {
    None,
    Ollama,
    LlamaCpp,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            backend: AiBackend::None,
            model_name: String::new(),
            endpoint: "http://localhost:11434".to_string(),
        }
    }
}

impl AiConfig {
    /// Check if Ollama is available on the system
    pub fn check_ollama_available() -> bool {
        std::process::Command::new("ollama")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// List available Ollama models
    #[allow(dead_code)]
    pub fn list_ollama_models() -> Vec<String> {
        if let Ok(output) = std::process::Command::new("ollama").arg("list").output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                return stdout
                    .lines()
                    .skip(1) // Skip header
                    .filter_map(|line| line.split_whitespace().next())
                    .map(String::from)
                    .collect();
            }
        }
        Vec::new()
    }

    /// Auto-start or install Ollama locally
    pub fn start_ollama() {
        if !Self::check_ollama_available() {
            if cfg!(unix) {
                // Spawn installation script in the background on Linux/macOS
                let _ = std::process::Command::new("sh")
                    .arg("-c")
                    .arg("curl -fsSL https://ollama.com/install.sh | sh")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();
            } else if cfg!(windows) {
                // On Windows, we can't easily auto-install, so we open the download page
                let _ = std::process::Command::new("cmd")
                    .arg("/c")
                    .arg("start https://ollama.com/download")
                    .spawn();
            }
        }

        // Start the server in the background
        let cmd = if cfg!(windows) {
            "ollama.exe"
        } else {
            "ollama"
        };
        let _ = std::process::Command::new(cmd)
            .arg("serve")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
    }

    /// Pull the specified AI model in the background
    pub fn pull_ollama_model(model: &str) {
        let _ = std::process::Command::new("ollama")
            .arg("pull")
            .arg(model)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
    }
}

/// Message roles for AI chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessage {
    pub role: String, // "user" or "assistant"
    pub content: String,
}

/// Response types from the background AI thread
pub enum AiResponse {
    Partial(String),
    Full(String),
    Error(String),
}

/// AI Assistant — handles interaction with the local model
pub struct AiAssistant {
    config: AiConfig,
}

impl AiAssistant {
    pub fn new(config: AiConfig) -> Self {
        Self { config }
    }

    pub fn is_available(&self) -> bool {
        self.config.enabled
            && matches!(self.config.backend, AiBackend::Ollama | AiBackend::LlamaCpp)
    }

    /// Generate a completion for the given context
    #[allow(dead_code)]
    pub fn complete(&self, context: &str) -> Option<String> {
        if !self.is_available() {
            return None;
        }

        match self.config.backend {
            AiBackend::Ollama => {
                let url = format!("{}/api/generate", self.config.endpoint);
                let body = serde_json::json!({
                    "model": self.config.model_name,
                    "prompt": context,
                    "stream": false
                });

                let response = ureq::post(&url).send_json(body).ok()?;

                let json: serde_json::Value = response.into_json().ok()?;
                json.get("response")
                    .and_then(|r| r.as_str())
                    .map(|s| s.to_string())
            }
            _ => None,
        }
    }

    /// Explain the selected code
    #[allow(dead_code)]
    pub fn explain(&self, code: &str) -> Option<String> {
        self.complete(&format!(
            "Please explain the following code concisely:\n\n{}",
            code
        ))
    }

    /// Generate a chat response in a background thread
    pub fn chat(&self, messages: Vec<AiMessage>, tx: std::sync::mpsc::Sender<AiResponse>) {
        if !self.is_available() {
            let _ = tx.send(AiResponse::Error("AI is disabled or unavailable".into()));
            return;
        }

        let config = self.config.clone();

        std::thread::spawn(move || match config.backend {
            AiBackend::Ollama => {
                let mut messages_json: Vec<serde_json::Value> = Vec::new();

                let project_context = if let Ok(cwd) = std::env::current_dir() {
                    format!("\n\nCurrent project directory: {}", cwd.display())
                } else {
                    String::new()
                };

                let system_prompt = format!(
                    "{}{}",
                    "You are Aether AI, a powerful autonomous coding assistant and vibecoding expert in the Aether Editor. \
                     You help users build entire projects from scratch using rapid prototyping and iterative development. \
                     Your job is to understand the user's intent and make it happen - code, create, run, and debug. \
                     \n\n## Available Commands\n\
                     Use these commands to interact with the filesystem and system. ALL commands require user approval.\n\n\
                     @@CREATE path/to/file.ext\n[file content]\n@@\n\
                       → Create a new file with content\n\n\
                     @@APPEND path/to/file.ext\n[content to add]\n@@\n\
                       → Append content to existing file\n\n\
                     @@READ path/to/file.ext\n\
                       → Read and display file contents\n\n\
                     @@DELETE path/to/file.ext\n\
                       → Delete a file\n\n\
                     @@RENAME old/path new/path\n\
                       → Rename or move a file\n\n\
                     @@LIST [optional/path]\n\
                       → List files in directory\n\n\
                     @@GREP pattern [path]\n\
                       → Search for pattern in files (like grep)\n\n\
                     @@SHELL command\n\
                       → Run shell command and get output (use for: npm install, cargo build, running servers, etc.)\n\n\
                     @@TEST [optional command]\n\
                       → Run tests (defaults to 'cargo test')\n\n\
                     @@COMMIT \"commit message\"\n\
                       → Git commit all changes\n\n\
                     @@WEBFETCH url\n\
                       → Fetch content from a URL (for documentation, APIs)\n\n\
                     \n## Vibecoding Guidelines\n\
                     - When user wants to build something, first explore the project structure with @@LIST\n\
                     - Create files proactively - don't ask permission for trivial files\n\
                     - Use @@SHELL to run builds, installs, and dev servers\n\
                     - Show the user what you created and how to run it\n\
                     - Be aggressive with scaffolding - generate boilerplate, config files, etc.\n\
                     - If something fails, read error output and fix it\n\
                     - Use markdown for code blocks and responses\n\n\
                     IMPORTANT: All file operations and shell commands are held for user approval in the sidebar.",
                    project_context
                );

                messages_json.push(serde_json::json!({
                    "role": "system",
                    "content": system_prompt
                }));

                for m in messages {
                    messages_json.push(serde_json::json!({
                        "role": m.role,
                        "content": m.content
                    }));
                }

                let url = format!("{}/api/chat", config.endpoint);
                let body = serde_json::json!({
                    "model": config.model_name,
                    "messages": messages_json,
                    "stream": true
                });

                let response = ureq::post(&url).send_json(body);

                match response {
                    Ok(resp) => {
                        let reader = resp.into_reader();
                        let decoder = serde_json::Deserializer::from_reader(reader)
                            .into_iter::<serde_json::Value>();

                        let mut full_response = String::new();

                        for value in decoder {
                            if let Ok(json) = value {
                                if let Some(content) = json
                                    .get("message")
                                    .and_then(|m| m.get("content"))
                                    .and_then(|c| c.as_str())
                                {
                                    full_response.push_str(content);
                                    let _ = tx.send(AiResponse::Partial(content.to_string()));
                                }
                                if let Some(done) = json.get("done").and_then(|d| d.as_bool()) {
                                    if done {
                                        break;
                                    }
                                }
                            }
                        }
                        let _ = tx.send(AiResponse::Full(full_response));
                    }
                    Err(e) => {
                        let _ = tx.send(AiResponse::Error(format!("Request failed: {}", e)));
                    }
                }
            }
            _ => {
                let _ = tx.send(AiResponse::Error("Unsupported backend".into()));
            }
        });
    }
}
