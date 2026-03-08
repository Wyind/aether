use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::{App, EditMode, VimSubMode};

pub fn draw_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;

    // Check if we have an active status message (show for 3 seconds)
    let show_status_msg = !app.status_message.is_empty()
        && app.status_message_time.elapsed().as_secs() < 3;

    // Build left section: mode indicator
    let mode_text = match &app.edit_mode {
        EditMode::Vim => match &app.vim_mode {
            VimSubMode::Normal => " NORMAL ",
            VimSubMode::Insert => " INSERT ",
            VimSubMode::Command => " COMMAND ",
        },
        EditMode::Nano => " NANO ",
        EditMode::Emacs => " EMACS ",
        EditMode::Aether => " AETHER ",
    };

    let mode_style = match &app.edit_mode {
        EditMode::Vim => match &app.vim_mode {
            VimSubMode::Normal => Style::default().fg(theme.bg).bg(theme.accent).add_modifier(Modifier::BOLD),
            VimSubMode::Insert => Style::default().fg(theme.bg).bg(theme.success).add_modifier(Modifier::BOLD),
            VimSubMode::Command => Style::default().fg(theme.bg).bg(theme.warning).add_modifier(Modifier::BOLD),
        },
        EditMode::Nano => Style::default().fg(theme.bg).bg(theme.string).add_modifier(Modifier::BOLD),
        EditMode::Emacs => Style::default().fg(theme.bg).bg(theme.accent).add_modifier(Modifier::BOLD),
        EditMode::Aether => Style::default().fg(theme.bg).bg(theme.accent).add_modifier(Modifier::BOLD),
    };

    // Vim command buffer display
    let cmd_display = if app.edit_mode == EditMode::Vim && app.vim_mode == VimSubMode::Command {
        format!(" :{}", app.vim_command_buffer)
    } else {
        String::new()
    };

    // Build middle section: filename
    let file_info = if !app.documents.is_empty() {
        let doc = &app.documents[app.active_tab];
        let modified = if doc.modified { " [+]" } else { "" };
        format!(" {}{} ", doc.file_name(), modified)
    } else {
        " No file ".to_string()
    };

    // Build right section: cursor pos, file type, encoding
    let right_info = if !app.documents.is_empty() {
        let doc = &app.documents[app.active_tab];
        format!(
            " Ln {}, Col {} │ {} │ UTF-8 ",
            doc.cursor.row + 1,
            doc.cursor.col + 1,
            doc.file_type,
        )
    } else {
        String::new()
    };

    // Status message or normal display
    let status_text = if show_status_msg {
        format!(" {} ", app.status_message)
    } else {
        String::new()
    };

    // Calculate widths
    let total_width = area.width as usize;
    let mode_len = mode_text.len();
    let cmd_len = cmd_display.len();
    let right_len = right_info.len();
    let status_len = status_text.len();
    let file_len = file_info.len();

    let left_content = mode_len + cmd_len;
    let mid_content = if show_status_msg { status_len } else { file_len };
    let padding = total_width.saturating_sub(left_content + mid_content + right_len);

    let mut spans = vec![
        Span::styled(mode_text, mode_style),
    ];

    if !cmd_display.is_empty() {
        spans.push(Span::styled(cmd_display, Style::default().fg(theme.warning).bg(theme.status_bg).add_modifier(Modifier::BOLD)));
    }

    if show_status_msg {
        spans.push(Span::styled(
            status_text,
            Style::default().fg(theme.accent).bg(theme.status_bg).add_modifier(Modifier::ITALIC),
        ));
    } else {
        spans.push(Span::styled(
            file_info,
            Style::default().fg(theme.status_fg).bg(theme.status_bg),
        ));
    }

    // Padding
    if padding > 0 {
        spans.push(Span::styled(
            " ".repeat(padding),
            Style::default().bg(theme.status_bg),
        ));
    }

    // Right section
    spans.push(Span::styled(
        right_info,
        Style::default().fg(theme.accent_dim).bg(theme.status_bg),
    ));

    let line = Line::from(spans);
    let bar = Paragraph::new(line);
    frame.render_widget(bar, area);
}
