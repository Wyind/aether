#[derive(Debug, Clone)]
pub struct Buffer {
    pub lines: Vec<String>,
    undo_stack: Vec<BufferSnapshot>,
    redo_stack: Vec<BufferSnapshot>,
}

#[derive(Debug, Clone)]
struct BufferSnapshot {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn from_text(text: &str) -> Self {
        let lines: Vec<String> = text.lines().map(String::from).collect();
        Self {
            lines: if lines.is_empty() { vec![String::new()] } else { lines },
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn save_snapshot(&mut self, cursor_row: usize, cursor_col: usize) {
        self.undo_stack.push(BufferSnapshot {
            lines: self.lines.clone(),
            cursor_row,
            cursor_col,
        });
        // Limit undo history
        if self.undo_stack.len() > 100 {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<(usize, usize)> {
        if let Some(snapshot) = self.undo_stack.pop() {
            self.redo_stack.push(BufferSnapshot {
                lines: self.lines.clone(),
                cursor_row: snapshot.cursor_row,
                cursor_col: snapshot.cursor_col,
            });
            self.lines = snapshot.lines;
            Some((snapshot.cursor_row, snapshot.cursor_col))
        } else {
            None
        }
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn line_len(&self, row: usize) -> usize {
        self.lines.get(row).map(|l| l.len()).unwrap_or(0)
    }

    pub fn get_line(&self, row: usize) -> &str {
        self.lines.get(row).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn insert_char(&mut self, row: usize, col: usize, c: char) {
        if row < self.lines.len() {
            let col = col.min(self.lines[row].len());
            self.lines[row].insert(col, c);
        }
    }

    pub fn delete_char(&mut self, row: usize, col: usize) -> bool {
        if row < self.lines.len() && col < self.lines[row].len() {
            self.lines[row].remove(col);
            true
        } else if row < self.lines.len() && col >= self.lines[row].len() && row + 1 < self.lines.len() {
            // Join with next line
            let next = self.lines.remove(row + 1);
            self.lines[row].push_str(&next);
            true
        } else {
            false
        }
    }

    pub fn backspace(&mut self, row: usize, col: usize) -> Option<(usize, usize)> {
        if col > 0 && row < self.lines.len() {
            let new_col = col - 1;
            self.lines[row].remove(new_col);
            Some((row, new_col))
        } else if row > 0 {
            // Join with previous line
            let current = self.lines.remove(row);
            let new_col = self.lines[row - 1].len();
            self.lines[row - 1].push_str(&current);
            Some((row - 1, new_col))
        } else {
            None
        }
    }

    pub fn split_line(&mut self, row: usize, col: usize) {
        if row < self.lines.len() {
            let col = col.min(self.lines[row].len());
            let rest = self.lines[row][col..].to_string();
            self.lines[row].truncate(col);
            self.lines.insert(row + 1, rest);
        }
    }

    pub fn insert_line(&mut self, row: usize, content: String) {
        if row <= self.lines.len() {
            self.lines.insert(row, content);
        }
    }

    pub fn delete_line(&mut self, row: usize) {
        if self.lines.len() > 1 && row < self.lines.len() {
            self.lines.remove(row);
        } else if self.lines.len() == 1 {
            self.lines[0].clear();
        }
    }

    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}
