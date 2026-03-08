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
        if let Ok(output) = std::process::Command::new("ollama")
            .arg("list")
            .output()
        {
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
        let cmd = if cfg!(windows) { "ollama.exe" } else { "ollama" };
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

/// AI Assistant — handles interaction with the local model
pub struct AiAssistant {
    config: AiConfig,
}

impl AiAssistant {
    pub fn new(config: AiConfig) -> Self {
        Self { config }
    }

    pub fn is_available(&self) -> bool {
        self.config.enabled && matches!(self.config.backend, AiBackend::Ollama | AiBackend::LlamaCpp)
    }

    /// Generate a completion for the given context
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

                let response = ureq::post(&url)
                    .send_json(body)
                    .ok()?;
                    
                let json: serde_json::Value = response.into_json().ok()?;
                json.get("response").and_then(|r| r.as_str()).map(|s| s.to_string())
            }
            _ => None
        }
    }

    /// Explain the selected code
    pub fn explain(&self, code: &str) -> Option<String> {
        self.complete(&format!("Please explain the following code concisely:\n\n{}", code))
    }
}

