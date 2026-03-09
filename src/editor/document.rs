use super::buffer::Buffer;
use super::cursor::Cursor;
use std::io;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Document {
    pub buffer: Buffer,
    pub cursor: Cursor,
    pub file_path: Option<String>,
    pub modified: bool,
    pub file_type: String,
}

impl Document {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            cursor: Cursor::new(),
            file_path: None,
            modified: false,
            file_type: "Plain Text".to_string(),
        }
    }

    pub fn open(path: &str) -> io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let file_type = detect_file_type(path);
        Ok(Self {
            buffer: Buffer::from_text(&content),
            cursor: Cursor::new(),
            file_path: Some(path.to_string()),
            modified: false,
            file_type,
        })
    }

    pub fn save(&mut self) -> io::Result<()> {
        if let Some(ref path) = self.file_path {
            std::fs::write(path, self.buffer.to_string())?;
            self.modified = false;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No file path set"))
        }
    }

    pub fn save_as(&mut self, path: &str) -> io::Result<()> {
        self.file_path = Some(path.to_string());
        self.file_type = detect_file_type(path);
        self.save()
    }

    pub fn file_name(&self) -> &str {
        self.file_path
            .as_ref()
            .and_then(|p| Path::new(p).file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("[untitled]")
    }

    pub fn insert_char(&mut self, c: char) {
        self.buffer.save_snapshot(self.cursor.row, self.cursor.col);
        self.buffer.insert_char(self.cursor.row, self.cursor.col, c);
        self.cursor.col += 1;
        self.cursor.desired_col = self.cursor.col;
        self.modified = true;
    }

    pub fn insert_char_no_move(&mut self, c: char) {
        self.buffer.insert_char(self.cursor.row, self.cursor.col, c);
    }

    pub fn insert_newline(&mut self) {
        self.buffer.save_snapshot(self.cursor.row, self.cursor.col);
        self.buffer.split_line(self.cursor.row, self.cursor.col);
        self.cursor.row += 1;
        self.cursor.col = 0;
        self.cursor.desired_col = 0;
        self.modified = true;
    }

    pub fn backspace(&mut self) {
        self.buffer.save_snapshot(self.cursor.row, self.cursor.col);
        if let Some((row, col)) = self.buffer.backspace(self.cursor.row, self.cursor.col) {
            self.cursor.row = row;
            self.cursor.col = col;
            self.cursor.desired_col = col;
            self.modified = true;
        }
    }

    pub fn delete_char(&mut self) {
        self.buffer.save_snapshot(self.cursor.row, self.cursor.col);
        if self.buffer.delete_char(self.cursor.row, self.cursor.col) {
            self.modified = true;
        }
    }

    pub fn delete_line(&mut self) {
        self.buffer.save_snapshot(self.cursor.row, self.cursor.col);
        self.buffer.delete_line(self.cursor.row);
        self.cursor.clamp(&self.buffer);
        self.modified = true;
    }

    pub fn insert_line_below(&mut self) {
        self.buffer.save_snapshot(self.cursor.row, self.cursor.col);
        let indent = self.get_current_indent();
        self.buffer.insert_line(self.cursor.row + 1, indent.clone());
        self.cursor.row += 1;
        self.cursor.col = indent.len();
        self.cursor.desired_col = self.cursor.col;
        self.modified = true;
    }

    pub fn insert_line_above(&mut self) {
        self.buffer.save_snapshot(self.cursor.row, self.cursor.col);
        let indent = self.get_current_indent();
        self.buffer.insert_line(self.cursor.row, indent.clone());
        self.cursor.col = indent.len();
        self.cursor.desired_col = self.cursor.col;
        self.modified = true;
    }

    pub fn undo(&mut self) {
        if let Some((row, col)) = self.buffer.undo() {
            self.cursor.row = row;
            self.cursor.col = col;
            self.cursor.desired_col = col;
        }
    }

    pub fn get_current_indent(&self) -> String {
        let line = self.buffer.get_line(self.cursor.row);
        let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
        indent
    }

    pub fn find_next(&mut self, query: &str) {
        let start_row = self.cursor.row;
        let start_col = self.cursor.col + 1;

        // Search from cursor position forward
        for row in start_row..self.buffer.line_count() {
            let line = self.buffer.get_line(row);
            let search_from = if row == start_row { start_col } else { 0 };
            if search_from < line.len() {
                if let Some(pos) = line[search_from..].find(query) {
                    self.cursor.row = row;
                    self.cursor.col = search_from + pos;
                    self.cursor.desired_col = self.cursor.col;
                    return;
                }
            }
        }

        // Wrap around
        for row in 0..=start_row {
            let line = self.buffer.get_line(row);
            if let Some(pos) = line.find(query) {
                self.cursor.row = row;
                self.cursor.col = pos;
                self.cursor.desired_col = self.cursor.col;
                return;
            }
        }
    }

    pub fn line_count(&self) -> usize {
        self.buffer.line_count()
    }

    pub fn char_count(&self) -> usize {
        self.buffer.to_string().chars().count()
    }
}

fn detect_file_type(path: &str) -> String {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match ext {
        "rs" => "Rust",
        "py" => "Python",
        "js" => "JavaScript",
        "ts" => "TypeScript",
        "jsx" | "tsx" => "React",
        "html" | "htm" => "HTML",
        "css" => "CSS",
        "json" => "JSON",
        "toml" => "TOML",
        "yaml" | "yml" => "YAML",
        "md" => "Markdown",
        "sh" | "bash" => "Shell",
        "c" => "C",
        "cpp" | "cc" | "cxx" => "C++",
        "h" | "hpp" => "Header",
        "go" => "Go",
        "java" => "Java",
        "rb" => "Ruby",
        "lua" => "Lua",
        "zig" => "Zig",
        "xml" => "XML",
        "sql" => "SQL",
        "txt" => "Text",
        _ => "Plain Text",
    }
    .to_string()
}
