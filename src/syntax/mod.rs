use std::sync::OnceLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use ratatui::style::{Color, Style, Modifier};
use ratatui::text::{Line, Span};

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

pub struct SyntaxHighlighter {
    // We don't need to store them in the struct anymore, just access the static caches
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        // Initialize if empty
        SYNTAX_SET.get_or_init(|| SyntaxSet::load_defaults_newlines());
        THEME_SET.get_or_init(|| ThemeSet::load_defaults());
        
        Self {}
    }

    pub fn highlight_line<'a>(
        &self,
        line_text: &str,
        file_ext: &str,
        theme: &crate::theme::Theme,
    ) -> Line<'a> {
        let syntax_set = SYNTAX_SET.get().unwrap();
        let theme_set = THEME_SET.get().unwrap();

        let syntax = syntax_set
            .find_syntax_by_extension(file_ext)
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

        let syntect_theme = &theme_set.themes["base16-ocean.dark"];
        let mut highlighter = HighlightLines::new(syntax, syntect_theme);

        let text_with_newline = if line_text.ends_with('\n') {
            line_text.to_string()
        } else {
            format!("{}\n", line_text)
        };

        match highlighter.highlight_line(&text_with_newline, syntax_set) {
            Ok(ranges) => {
                let spans: Vec<Span<'a>> = ranges.iter().map(|(style, text)| {
                    let color = syntect_to_ratatui_color(style.foreground, theme);
                    let mut ratatui_style = Style::default().fg(color);
                    if style.font_style.contains(syntect::highlighting::FontStyle::BOLD) {
                        ratatui_style = ratatui_style.add_modifier(Modifier::BOLD);
                    }
                    if style.font_style.contains(syntect::highlighting::FontStyle::ITALIC) {
                        ratatui_style = ratatui_style.add_modifier(Modifier::ITALIC);
                    }
                    Span::styled(text.trim_end_matches('\n').to_string(), ratatui_style)
                }).collect();
                Line::from(spans)
            }
            Err(_) => {
                Line::from(Span::styled(
                    line_text.to_string(),
                    Style::default().fg(theme.fg),
                ))
            }
        }
    }
}

fn syntect_to_ratatui_color(
    syntect_color: syntect::highlighting::Color,
    _theme: &crate::theme::Theme,
) -> Color {
    Color::Rgb(syntect_color.r, syntect_color.g, syntect_color.b)
}

pub fn get_file_extension(file_type: &str) -> &str {
    match file_type {
        "Rust" => "rs",
        "Python" => "py",
        "JavaScript" => "js",
        "TypeScript" => "ts",
        "React" => "tsx",
        "HTML" => "html",
        "CSS" => "css",
        "JSON" => "json",
        "TOML" => "toml",
        "YAML" => "yaml",
        "Markdown" => "md",
        "Shell" => "sh",
        "C" => "c",
        "C++" => "cpp",
        "Header" => "h",
        "Go" => "go",
        "Java" => "java",
        "Ruby" => "rb",
        "Lua" => "lua",
        "Zig" => "zig",
        "XML" => "xml",
        "SQL" => "sql",
        _ => "txt",
    }
}
