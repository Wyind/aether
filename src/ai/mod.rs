// AI Module — AI support for code assistance
//
// Aether supports both local and online AI models for code completion,
// explanation, and vibecoding.
//
// Local (runs on your machine):
//   - Ollama (llama3, codellama, mistral, etc.)
//   - llama.cpp via GGUF models
//
// Online (requires API key):
//   - OpenAI (GPT-4, GPT-4o, GPT-3.5)
//   - Anthropic (Claude 3.5, Claude 3)
//   - Google Gemini
//   - xAI Grok

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub enabled: bool,
    pub backend: AiBackend,
    pub model_name: String,
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AiBackend {
    None,
    // Local
    Ollama,
    LlamaCpp,
    // Online
    OpenAI,
    Anthropic,
    Gemini,
    Grok,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            backend: AiBackend::None,
            model_name: String::new(),
            endpoint: String::new(),
            api_key: None,
        }
    }
}

impl AiConfig {
    pub fn is_online(&self) -> bool {
        matches!(
            self.backend,
            AiBackend::OpenAI | AiBackend::Anthropic | AiBackend::Gemini | AiBackend::Grok
        )
    }

    pub fn is_local(&self) -> bool {
        matches!(self.backend, AiBackend::Ollama | AiBackend::LlamaCpp)
    }

    pub fn needs_api_key(&self) -> bool {
        self.is_online()
    }

    pub fn has_valid_api_key(&self) -> bool {
        self.api_key
            .as_ref()
            .map(|k| !k.is_empty())
            .unwrap_or(false)
    }

    pub fn check_ollama_available() -> bool {
        std::process::Command::new("ollama")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    #[allow(dead_code)]
    pub fn list_ollama_models() -> Vec<String> {
        if let Ok(output) = std::process::Command::new("ollama").arg("list").output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                return stdout
                    .lines()
                    .skip(1)
                    .filter_map(|line| line.split_whitespace().next())
                    .map(String::from)
                    .collect();
            }
        }
        Vec::new()
    }

    pub fn start_ollama() {
        if !Self::check_ollama_available() {
            if cfg!(unix) {
                let _ = std::process::Command::new("sh")
                    .arg("-c")
                    .arg("curl -fsSL https://ollama.com/install.sh | sh")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();
            } else if cfg!(windows) {
                let _ = std::process::Command::new("cmd")
                    .arg("/c")
                    .arg("start https://ollama.com/download")
                    .spawn();
            }
        }

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

    pub fn pull_ollama_model(model: &str) {
        let _ = std::process::Command::new("ollama")
            .arg("pull")
            .arg(model)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
    }

    pub fn verify_api_key(backend: &AiBackend, api_key: &str, _endpoint: &str) -> bool {
        if api_key.is_empty() {
            return false;
        }

        match backend {
            AiBackend::OpenAI => {
                let client = reqwest::blocking::Client::new();
                let result: Result<reqwest::blocking::Response, _> = client
                    .get("https://api.openai.com/v1/models")
                    .header("Authorization", format!("Bearer {}", api_key))
                    .send();
                result.map(|r| r.status().is_success()).unwrap_or(false)
            }
            AiBackend::Anthropic => {
                let client = reqwest::blocking::Client::new();
                let result: Result<reqwest::blocking::Response, _> = client.post("https://api.anthropic.com/v1/messages")
                    .header("x-api-key", api_key)
                    .header("anthropic-version", "2023-06-01")
                    .header("Content-Type", "application/json")
                    .body(r#"{"model":"claude-3-haiku-20240307","max_tokens":1,"messages":[{"role":"user","content":"hi"}]}"#)
                    .send();
                result
                    .map(|r| r.status().is_success() || r.status().as_u16() == 400)
                    .unwrap_or(false)
            }
            AiBackend::Gemini => {
                let url = format!(
                    "https://generativelanguage.googleapis.com/v1/models?key={}",
                    api_key
                );
                let client = reqwest::blocking::Client::new();
                let result: Result<reqwest::blocking::Response, _> = client.get(&url).send();
                result.map(|r| r.status().is_success()).unwrap_or(false)
            }
            AiBackend::Grok => {
                let client = reqwest::blocking::Client::new();
                let result: Result<reqwest::blocking::Response, _> = client
                    .get("https://api.x.ai")
                    .header("Authorization", format!("Bearer {}", api_key))
                    .send();
                result.map(|r| r.status().is_success()).unwrap_or(false)
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessage {
    pub role: String,
    pub content: String,
}

pub enum AiResponse {
    Partial(String),
    Full(String),
    Error(String),
}

pub struct AiAssistant {
    config: AiConfig,
}

impl AiAssistant {
    pub fn new(config: AiConfig) -> Self {
        Self { config }
    }

    pub fn is_available(&self) -> bool {
        if !self.config.enabled {
            return false;
        }

        if self.config.is_online() {
            self.config.has_valid_api_key()
        } else {
            matches!(self.config.backend, AiBackend::Ollama | AiBackend::LlamaCpp)
        }
    }

    pub fn get_system_prompt(&self) -> String {
        let project_context = std::env::current_dir()
            .map(|p| format!("\n\nCurrent project directory: {}", p.display()))
            .unwrap_or_default();

        format!(
            "{}{}",
            r#"You are Aether AI, a powerful autonomous coding assistant and vibecoding expert in the Aether Editor. 
You help users build entire projects from scratch using rapid prototyping and iterative development. 
Your job is to understand the user's intent and make it happen - code, create, run, and debug. 

## Available Commands
Use these commands to interact with the filesystem and system. ALL commands require user approval.

@@CREATE path/to/file.ext
[file content]
@@
  → Create a new file with content

@@APPEND path/to/file.ext
[content to add]
@@
  → Append content to existing file

@@READ path/to/file.ext
  → Read and display file contents

@@DELETE path/to/file.ext
  → Delete a file

@@RENAME old/path new/path
  → Rename or move a file

@@LIST [optional/path]
  → List files in directory

@@GREP pattern [path]
  → Search for pattern in files (like grep)

@@SHELL command
  → Run shell command and get output (use for: npm install, cargo build, running servers, etc.)

@@TEST [optional command]
  → Run tests (defaults to 'cargo test')

@@COMMIT "commit message"
  → Git commit all changes

@@WEBFETCH url
  → Fetch content from a URL (for documentation, APIs)

## Vibecoding Guidelines
- When user wants to build something, first explore the project structure with @@LIST
- Create files proactively - don't ask permission for trivial files
- Use @@SHELL to run builds, installs, and dev servers
- Show the user what you created and how to run it
- Be aggressive with scaffolding - generate boilerplate, config files, etc.
- If something fails, read error output and fix it
- Use markdown for code blocks and responses

IMPORTANT: All file operations and shell commands are held for user approval in the sidebar."#,
            project_context
        )
    }

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

    pub fn chat(&self, messages: Vec<AiMessage>, tx: std::sync::mpsc::Sender<AiResponse>) {
        if !self.is_available() {
            let _ = tx.send(AiResponse::Error("AI is disabled or unavailable".into()));
            return;
        }

        let config = self.config.clone();
        let system_prompt = self.get_system_prompt();

        std::thread::spawn(move || match config.backend {
            AiBackend::Ollama => {
                Self::ollama_chat(&config, messages, tx, &system_prompt);
            }
            AiBackend::OpenAI => {
                Self::openai_chat(&config, messages, tx, &system_prompt);
            }
            AiBackend::Anthropic => {
                Self::anthropic_chat(&config, messages, tx, &system_prompt);
            }
            AiBackend::Gemini => {
                Self::gemini_chat(&config, messages, tx, &system_prompt);
            }
            AiBackend::Grok => {
                Self::grok_chat(&config, messages, tx, &system_prompt);
            }
            _ => {
                let _ = tx.send(AiResponse::Error("Unsupported backend".into()));
            }
        });
    }

    fn ollama_chat(
        config: &AiConfig,
        messages: Vec<AiMessage>,
        tx: std::sync::mpsc::Sender<AiResponse>,
        system_prompt: &str,
    ) {
        let mut messages_json: Vec<serde_json::Value> = Vec::new();

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
                let decoder =
                    serde_json::Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

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
                let _ = tx.send(AiResponse::Error(format!("Ollama request failed: {}", e)));
            }
        }
    }

    fn openai_chat(
        config: &AiConfig,
        messages: Vec<AiMessage>,
        tx: std::sync::mpsc::Sender<AiResponse>,
        system_prompt: &str,
    ) {
        let api_key = match &config.api_key {
            Some(k) => k,
            None => {
                let _ = tx.send(AiResponse::Error("OpenAI API key not configured".into()));
                return;
            }
        };

        let mut messages_json: Vec<serde_json::Value> = Vec::new();
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

        let client = reqwest::blocking::Client::new();
        let url = "https://api.openai.com/v1/chat/completions";

        let body = serde_json::json!({
            "model": if config.model_name.is_empty() { "gpt-4o" } else { &config.model_name },
            "messages": messages_json,
            "stream": true
        });

        let result: Result<reqwest::blocking::Response, reqwest::Error> = client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send();

        match result {
            Ok(resp) => {
                let status = resp.status();
                if !status.is_success() {
                    let status = resp.status();
                    let body = resp.text().unwrap_or_default();
                    let _ = tx.send(AiResponse::Error(format!(
                        "OpenAI API error ({}): {}",
                        status, body
                    )));
                    return;
                }

                let body = resp.text().unwrap_or_default();
                let mut full_response = String::new();

                for line in body.lines() {
                    if line.starts_with("data: ") {
                        let data = line.trim_start_matches("data: ");
                        if data == "[DONE]" {
                            break;
                        }
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                            if let Some(choices) = json.get("choices").and_then(|c| c.as_array()) {
                                if let Some(choice) = choices.first() {
                                    if let Some(content) = choice
                                        .get("delta")
                                        .and_then(|d| d.get("content"))
                                        .and_then(|c| c.as_str())
                                    {
                                        full_response.push_str(content);
                                        let _ = tx.send(AiResponse::Partial(content.to_string()));
                                    }
                                }
                            }
                        }
                    }
                }
                let _ = tx.send(AiResponse::Full(full_response));
            }
            Err(e) => {
                let _ = tx.send(AiResponse::Error(format!("OpenAI request failed: {}", e)));
            }
        }
    }

    fn anthropic_chat(
        config: &AiConfig,
        messages: Vec<AiMessage>,
        tx: std::sync::mpsc::Sender<AiResponse>,
        system_prompt: &str,
    ) {
        let api_key = match &config.api_key {
            Some(k) => k,
            None => {
                let _ = tx.send(AiResponse::Error("Anthropic API key not configured".into()));
                return;
            }
        };

        let model = if config.model_name.is_empty() {
            "claude-sonnet-4-20250514"
        } else {
            &config.model_name
        };

        let mut contents: Vec<serde_json::Value> = Vec::new();

        for m in &messages {
            let role = if m.role == "assistant" {
                "assistant"
            } else {
                "user"
            };
            contents.push(serde_json::json!({
                "type": "text",
                "text": m.content
            }));
        }

        let client = reqwest::blocking::Client::new();
        let url = "https://api.anthropic.com/v1/messages";

        let body = serde_json::json!({
            "model": model,
            "max_tokens": 4096,
            "system": system_prompt,
            "messages": contents
        });

        let result: Result<reqwest::blocking::Response, reqwest::Error> = client
            .post(url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send();

        match result {
            Ok(resp) => {
                let status = resp.status();
                if !status.is_success() {
                    let status = resp.status();
                    let body = resp.text().unwrap_or_default();
                    let _ = tx.send(AiResponse::Error(format!(
                        "Anthropic API error ({}): {}",
                        status, body
                    )));
                    return;
                }

                match resp.json::<serde_json::Value>() {
                    Ok(json) => {
                        let mut full_response = String::new();

                        if let Some(content) = json
                            .get("content")
                            .and_then(|c| c.as_array())
                            .and_then(|a| a.first())
                        {
                            if let Some(text) = content.get("text").and_then(|t| t.as_str()) {
                                full_response = text.to_string();
                                let _ = tx.send(AiResponse::Partial(text.to_string()));
                            }
                        }

                        let _ = tx.send(AiResponse::Full(full_response));
                    }
                    Err(e) => {
                        let _ = tx.send(AiResponse::Error(format!(
                            "Failed to parse Anthropic response: {}",
                            e
                        )));
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(AiResponse::Error(format!(
                    "Anthropic request failed: {}",
                    e
                )));
            }
        }
    }

    fn gemini_chat(
        config: &AiConfig,
        messages: Vec<AiMessage>,
        tx: std::sync::mpsc::Sender<AiResponse>,
        system_prompt: &str,
    ) {
        let api_key = match &config.api_key {
            Some(k) => k,
            None => {
                let _ = tx.send(AiResponse::Error("Gemini API key not configured".into()));
                return;
            }
        };

        let model = if config.model_name.is_empty() {
            "gemini-2.0-flash"
        } else {
            &config.model_name
        };

        let mut contents: Vec<serde_json::Value> = Vec::new();
        contents.push(serde_json::json!({
            "role": "user",
            "parts": [{"text": system_prompt}]
        }));

        for m in messages {
            let role = if m.role == "assistant" {
                "model"
            } else {
                "user"
            };
            contents.push(serde_json::json!({
                "role": role,
                "parts": [{"text": m.content}]
            }));
        }

        let client = reqwest::blocking::Client::new();
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?alt=sse&key={}",
            model, api_key
        );

        let body = serde_json::json!({
            "contents": contents,
            "generationConfig": {
                "temperature": 0.7,
                "maxOutputTokens": 4096
            }
        });

        let result: Result<reqwest::blocking::Response, reqwest::Error> = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send();

        match result {
            Ok(resp) => {
                let status = resp.status();
                if !status.is_success() {
                    let status = resp.status();
                    let body = resp.text().unwrap_or_default();
                    let _ = tx.send(AiResponse::Error(format!(
                        "Gemini API error ({}): {}",
                        status, body
                    )));
                    return;
                }

                let body = resp.text().unwrap_or_default();
                let mut full_response = String::new();

                for line in body.lines() {
                    if line.starts_with("data: ") {
                        let data = line.trim_start_matches("data: ");
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                            if let Some(candidates) =
                                json.get("candidates").and_then(|c| c.as_array())
                            {
                                if let Some(candidate) = candidates.first() {
                                    if let Some(content) = candidate
                                        .get("content")
                                        .and_then(|c| c.get("parts"))
                                        .and_then(|p| p.as_array())
                                        .and_then(|a| a.first())
                                    {
                                        if let Some(text) =
                                            content.get("text").and_then(|t| t.as_str())
                                        {
                                            full_response.push_str(text);
                                            let _ = tx.send(AiResponse::Partial(text.to_string()));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                let _ = tx.send(AiResponse::Full(full_response));
            }
            Err(e) => {
                let _ = tx.send(AiResponse::Error(format!("Gemini request failed: {}", e)));
            }
        }
    }

    fn grok_chat(
        config: &AiConfig,
        messages: Vec<AiMessage>,
        tx: std::sync::mpsc::Sender<AiResponse>,
        system_prompt: &str,
    ) {
        let api_key = match &config.api_key {
            Some(k) => k,
            None => {
                let _ = tx.send(AiResponse::Error("Grok API key not configured".into()));
                return;
            }
        };

        let model = if config.model_name.is_empty() {
            "grok-3-mini-fast"
        } else {
            &config.model_name
        };

        let mut messages_json: Vec<serde_json::Value> = Vec::new();
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

        let client = reqwest::blocking::Client::new();
        let url = "https://api.x.ai/v1/chat/completions";

        let body = serde_json::json!({
            "model": model,
            "messages": messages_json,
            "stream": true
        });

        let result: Result<reqwest::blocking::Response, reqwest::Error> = client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send();

        match result {
            Ok(resp) => {
                let status = resp.status();
                if !status.is_success() {
                    let status = resp.status();
                    let body = resp.text().unwrap_or_default();
                    let _ = tx.send(AiResponse::Error(format!(
                        "Grok API error ({}): {}",
                        status, body
                    )));
                    return;
                }

                let body = resp.text().unwrap_or_default();
                let mut full_response = String::new();

                for line in body.lines() {
                    if line.starts_with("data: ") {
                        let data = line.trim_start_matches("data: ");
                        if data == "[DONE]" {
                            break;
                        }
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                            if let Some(choices) = json.get("choices").and_then(|c| c.as_array()) {
                                if let Some(choice) = choices.first() {
                                    if let Some(content) = choice
                                        .get("delta")
                                        .and_then(|d| d.get("content"))
                                        .and_then(|c| c.as_str())
                                    {
                                        full_response.push_str(content);
                                        let _ = tx.send(AiResponse::Partial(content.to_string()));
                                    }
                                }
                            }
                        }
                    }
                }
                let _ = tx.send(AiResponse::Full(full_response));
            }
            Err(e) => {
                let _ = tx.send(AiResponse::Error(format!("Grok request failed: {}", e)));
            }
        }
    }
}
