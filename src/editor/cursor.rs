use super::buffer::Buffer;

#[derive(Debug, Clone)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
    pub scroll_row: usize,
    pub scroll_col: usize,
    pub desired_col: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            scroll_row: 0,
            scroll_col: 0,
            desired_col: 0,
        }
    }

    pub fn move_up(&mut self) {
        if self.row > 0 {
            self.row -= 1;
            self.col = self.desired_col;
        }
    }

    pub fn move_down(&mut self, buffer: &Buffer) {
        if self.row + 1 < buffer.line_count() {
            self.row += 1;
            self.col = self.desired_col;
        }
    }

    pub fn move_left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
            self.desired_col = self.col;
        }
    }

    pub fn move_right(&mut self, buffer: &Buffer) {
        if self.col < buffer.line_len(self.row) {
            self.col += 1;
            self.desired_col = self.col;
        }
    }

    pub fn move_word_forward(&mut self, buffer: &Buffer) {
        let line = buffer.get_line(self.row);
        let chars: Vec<char> = line.chars().collect();
        let mut col = self.col;

        // Skip current word
        while col < chars.len() && !chars[col].is_whitespace() {
            col += 1;
        }
        // Skip whitespace
        while col < chars.len() && chars[col].is_whitespace() {
            col += 1;
        }

        if col <= chars.len() {
            self.col = col;
            self.desired_col = self.col;
        } else if self.row + 1 < buffer.line_count() {
            self.row += 1;
            self.col = 0;
            self.desired_col = 0;
        }
    }

    pub fn move_word_backward(&mut self, buffer: &Buffer) {
        if self.col == 0 {
            if self.row > 0 {
                self.row -= 1;
                self.col = buffer.line_len(self.row);
                self.desired_col = self.col;
            }
            return;
        }

        let line = buffer.get_line(self.row);
        let chars: Vec<char> = line.chars().collect();
        let mut col = self.col;

        // Skip whitespace backward
        while col > 0 && chars.get(col - 1).map_or(false, |c| c.is_whitespace()) {
            col -= 1;
        }
        // Skip word backward
        while col > 0 && chars.get(col - 1).map_or(false, |c| !c.is_whitespace()) {
            col -= 1;
        }

        self.col = col;
        self.desired_col = self.col;
    }

    pub fn move_to_end_of_line(&mut self, buffer: &Buffer) {
        self.col = buffer.line_len(self.row);
        self.desired_col = self.col;
    }

    pub fn move_to_bottom(&mut self, buffer: &Buffer) {
        self.row = buffer.line_count().saturating_sub(1);
        self.col = 0;
        self.desired_col = 0;
    }

    /// Clamp cursor position to valid range within buffer
    pub fn clamp(&mut self, buffer: &Buffer) {
        if self.row >= buffer.line_count() {
            self.row = buffer.line_count().saturating_sub(1);
        }
        let line_len = buffer.line_len(self.row);
        if self.col > line_len {
            self.col = line_len;
        }
    }

    /// Update scroll position based on viewport size
    pub fn update_scroll(&mut self, viewport_height: usize, viewport_width: usize) {
        // Vertical scroll
        if self.row < self.scroll_row {
            self.scroll_row = self.row;
        }
        if self.row >= self.scroll_row + viewport_height {
            self.scroll_row = self.row - viewport_height + 1;
        }

        // Horizontal scroll
        if self.col < self.scroll_col {
            self.scroll_col = self.col;
        }
        if self.col >= self.scroll_col + viewport_width {
            self.scroll_col = self.col - viewport_width + 1;
        }
    }
}
