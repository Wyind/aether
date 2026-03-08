use super::Theme;
use ratatui::style::Color;

/// Aether Dark — the signature theme
/// Deep dark with teal/cyan accents
pub fn aether_dark() -> Theme {
    Theme {
        name: "Aether Dark".to_string(),
        bg: Color::Rgb(5, 11, 20),             // #050B14
        fg: Color::Rgb(160, 255, 242),          // #A0FFF2
        accent: Color::Rgb(0, 255, 210),        // #00FFD2 accent glow
        accent_dim: Color::Rgb(60, 180, 170),   // #3CB4AA letter mid
        gutter_bg: Color::Rgb(3, 8, 16),
        gutter_fg: Color::Rgb(28, 110, 105),    // #1C6E69 letter dim
        active_line_bg: Color::Rgb(10, 20, 35),
        selection_bg: Color::Rgb(20, 50, 55),
        status_bg: Color::Rgb(8, 16, 28),
        status_fg: Color::Rgb(160, 255, 242),   // #A0FFF2
        tab_bg: Color::Rgb(3, 8, 16),
        tab_active_bg: Color::Rgb(10, 20, 35),
        tab_fg: Color::Rgb(28, 110, 105),       // #1C6E69
        tab_active_fg: Color::Rgb(160, 255, 242),// #A0FFF2
        sidebar_bg: Color::Rgb(3, 8, 16),
        sidebar_fg: Color::Rgb(75, 195, 185),   // #4BC3B9 tagline
        sidebar_active_bg: Color::Rgb(10, 20, 35),
        border: Color::Rgb(0, 215, 195),        // #00D7C3 border/frame
        keyword: Color::Rgb(0, 255, 210),       // #00FFD2 teal glow
        string: Color::Rgb(158, 206, 106),      // green
        comment: Color::Rgb(28, 110, 105),      // #1C6E69 dim
        function: Color::Rgb(125, 207, 255),    // cyan
        r#type: Color::Rgb(75, 195, 185),       // #4BC3B9
        number: Color::Rgb(255, 158, 100),      // orange
        operator: Color::Rgb(60, 180, 170),     // #3CB4AA
        error: Color::Rgb(247, 118, 142),       // red-pink
        warning: Color::Rgb(224, 175, 104),     // yellow
        success: Color::Rgb(158, 206, 106),     // green
        popup_bg: Color::Rgb(8, 16, 28),
        popup_border: Color::Rgb(0, 215, 195),  // #00D7C3
    }
}

/// Aether Light — clean and professional
pub fn aether_light() -> Theme {
    Theme {
        name: "Aether Light".to_string(),
        bg: Color::Rgb(245, 250, 252),          // #F5FAFC
        fg: Color::Rgb(0, 100, 95),             // #00645F letter dim
        accent: Color::Rgb(0, 175, 160),        // #00AFA0 accent glow
        accent_dim: Color::Rgb(0, 140, 130),    // #008C82 letter mid
        gutter_bg: Color::Rgb(238, 244, 248),
        gutter_fg: Color::Rgb(0, 130, 120),     // #008278 tagline
        active_line_bg: Color::Rgb(230, 245, 245),
        selection_bg: Color::Rgb(200, 235, 232),
        status_bg: Color::Rgb(232, 242, 244),
        status_fg: Color::Rgb(0, 100, 95),      // #00645F
        tab_bg: Color::Rgb(238, 244, 248),
        tab_active_bg: Color::Rgb(245, 250, 252),// #F5FAFC
        tab_fg: Color::Rgb(0, 130, 120),        // #008278
        tab_active_fg: Color::Rgb(0, 100, 95),  // #00645F
        sidebar_bg: Color::Rgb(238, 244, 248),
        sidebar_fg: Color::Rgb(0, 130, 120),    // #008278
        sidebar_active_bg: Color::Rgb(225, 240, 238),
        border: Color::Rgb(0, 155, 140),        // #009B8C border/frame
        keyword: Color::Rgb(0, 180, 168),       // #00B4A8
        string: Color::Rgb(72, 145, 50),        // green
        comment: Color::Rgb(0, 130, 120),       // #008278
        function: Color::Rgb(0, 155, 140),      // #009B8C
        r#type: Color::Rgb(0, 140, 130),        // #008C82
        number: Color::Rgb(200, 100, 30),       // orange
        operator: Color::Rgb(0, 100, 95),       // #00645F
        error: Color::Rgb(200, 50, 60),
        warning: Color::Rgb(200, 140, 30),
        success: Color::Rgb(72, 145, 50),
        popup_bg: Color::Rgb(250, 252, 254),
        popup_border: Color::Rgb(0, 155, 140),  // #009B8C
    }
}

/// Ember — warm Catppuccin-inspired theme
pub fn ember() -> Theme {
    Theme {
        name: "Ember".to_string(),
        bg: Color::Rgb(30, 30, 46),            // #1e1e2e
        fg: Color::Rgb(205, 214, 244),         // #cdd6f4
        accent: Color::Rgb(243, 139, 168),     // #f38ba8 pink
        accent_dim: Color::Rgb(180, 110, 130),
        gutter_bg: Color::Rgb(24, 24, 37),
        gutter_fg: Color::Rgb(88, 91, 112),
        active_line_bg: Color::Rgb(45, 45, 62),
        selection_bg: Color::Rgb(69, 71, 90),
        status_bg: Color::Rgb(24, 24, 37),
        status_fg: Color::Rgb(186, 194, 222),
        tab_bg: Color::Rgb(24, 24, 37),
        tab_active_bg: Color::Rgb(30, 30, 46),
        tab_fg: Color::Rgb(88, 91, 112),
        tab_active_fg: Color::Rgb(205, 214, 244),
        sidebar_bg: Color::Rgb(24, 24, 37),
        sidebar_fg: Color::Rgb(166, 173, 200),
        sidebar_active_bg: Color::Rgb(45, 45, 62),
        border: Color::Rgb(49, 50, 68),
        keyword: Color::Rgb(203, 166, 247),    // mauve
        string: Color::Rgb(166, 227, 161),     // green
        comment: Color::Rgb(108, 112, 134),
        function: Color::Rgb(137, 180, 250),   // blue
        r#type: Color::Rgb(249, 226, 175),     // yellow
        number: Color::Rgb(250, 179, 135),     // peach
        operator: Color::Rgb(148, 226, 213),   // teal
        error: Color::Rgb(243, 139, 168),
        warning: Color::Rgb(249, 226, 175),
        success: Color::Rgb(166, 227, 161),
        popup_bg: Color::Rgb(36, 36, 54),
        popup_border: Color::Rgb(243, 139, 168),
    }
}

/// Frost — Nord-inspired cool blues
pub fn frost() -> Theme {
    Theme {
        name: "Frost".to_string(),
        bg: Color::Rgb(46, 52, 64),            // #2e3440
        fg: Color::Rgb(216, 222, 233),         // #d8dee9
        accent: Color::Rgb(136, 192, 208),     // #88c0d0
        accent_dim: Color::Rgb(100, 145, 160),
        gutter_bg: Color::Rgb(40, 46, 56),
        gutter_fg: Color::Rgb(76, 86, 106),
        active_line_bg: Color::Rgb(59, 66, 82),
        selection_bg: Color::Rgb(67, 76, 94),
        status_bg: Color::Rgb(59, 66, 82),
        status_fg: Color::Rgb(216, 222, 233),
        tab_bg: Color::Rgb(40, 46, 56),
        tab_active_bg: Color::Rgb(46, 52, 64),
        tab_fg: Color::Rgb(76, 86, 106),
        tab_active_fg: Color::Rgb(216, 222, 233),
        sidebar_bg: Color::Rgb(40, 46, 56),
        sidebar_fg: Color::Rgb(180, 190, 205),
        sidebar_active_bg: Color::Rgb(59, 66, 82),
        border: Color::Rgb(59, 66, 82),
        keyword: Color::Rgb(180, 142, 173),    // purple
        string: Color::Rgb(163, 190, 140),     // green
        comment: Color::Rgb(97, 110, 136),
        function: Color::Rgb(136, 192, 208),   // cyan
        r#type: Color::Rgb(129, 161, 193),     // blue
        number: Color::Rgb(208, 135, 112),     // orange
        operator: Color::Rgb(143, 188, 187),   // teal
        error: Color::Rgb(191, 97, 106),
        warning: Color::Rgb(235, 203, 139),
        success: Color::Rgb(163, 190, 140),
        popup_bg: Color::Rgb(52, 58, 72),
        popup_border: Color::Rgb(136, 192, 208),
    }
}

/// Midnight — GitHub Dark-inspired
pub fn midnight() -> Theme {
    Theme {
        name: "Midnight".to_string(),
        bg: Color::Rgb(13, 17, 23),
        fg: Color::Rgb(201, 209, 217),
        accent: Color::Rgb(240, 136, 62),      // orange
        accent_dim: Color::Rgb(180, 105, 50),
        gutter_bg: Color::Rgb(10, 14, 20),
        gutter_fg: Color::Rgb(72, 80, 92),
        active_line_bg: Color::Rgb(22, 27, 34),
        selection_bg: Color::Rgb(38, 48, 62),
        status_bg: Color::Rgb(22, 27, 34),
        status_fg: Color::Rgb(201, 209, 217),
        tab_bg: Color::Rgb(10, 14, 20),
        tab_active_bg: Color::Rgb(13, 17, 23),
        tab_fg: Color::Rgb(72, 80, 92),
        tab_active_fg: Color::Rgb(201, 209, 217),
        sidebar_bg: Color::Rgb(10, 14, 20),
        sidebar_fg: Color::Rgb(150, 160, 175),
        sidebar_active_bg: Color::Rgb(22, 27, 34),
        border: Color::Rgb(33, 38, 45),
        keyword: Color::Rgb(255, 123, 114),
        string: Color::Rgb(165, 214, 255),
        comment: Color::Rgb(139, 148, 158),
        function: Color::Rgb(210, 168, 255),
        r#type: Color::Rgb(126, 231, 135),
        number: Color::Rgb(121, 192, 255),
        operator: Color::Rgb(255, 123, 114),
        error: Color::Rgb(248, 81, 73),
        warning: Color::Rgb(210, 153, 34),
        success: Color::Rgb(63, 185, 80),
        popup_bg: Color::Rgb(18, 22, 30),
        popup_border: Color::Rgb(240, 136, 62),
    }
}

/// Sakura — soft pink/purple Japanese-inspired
pub fn sakura() -> Theme {
    Theme {
        name: "Sakura".to_string(),
        bg: Color::Rgb(25, 20, 30),
        fg: Color::Rgb(220, 210, 230),
        accent: Color::Rgb(255, 145, 175),     // cherry blossom pink
        accent_dim: Color::Rgb(180, 100, 130),
        gutter_bg: Color::Rgb(20, 16, 25),
        gutter_fg: Color::Rgb(80, 65, 95),
        active_line_bg: Color::Rgb(35, 28, 45),
        selection_bg: Color::Rgb(60, 45, 75),
        status_bg: Color::Rgb(20, 16, 25),
        status_fg: Color::Rgb(220, 210, 230),
        tab_bg: Color::Rgb(20, 16, 25),
        tab_active_bg: Color::Rgb(25, 20, 30),
        tab_fg: Color::Rgb(80, 65, 95),
        tab_active_fg: Color::Rgb(255, 145, 175),
        sidebar_bg: Color::Rgb(20, 16, 25),
        sidebar_fg: Color::Rgb(180, 168, 195),
        sidebar_active_bg: Color::Rgb(35, 28, 45),
        border: Color::Rgb(60, 45, 75),
        keyword: Color::Rgb(255, 145, 175),    // pink
        string: Color::Rgb(150, 220, 180),     // mint green
        comment: Color::Rgb(100, 85, 115),
        function: Color::Rgb(180, 160, 255),   // lavender
        r#type: Color::Rgb(255, 200, 140),     // warm peach
        number: Color::Rgb(140, 200, 255),     // sky blue
        operator: Color::Rgb(220, 180, 220),
        error: Color::Rgb(255, 100, 110),
        warning: Color::Rgb(255, 200, 100),
        success: Color::Rgb(150, 220, 180),
        popup_bg: Color::Rgb(30, 24, 38),
        popup_border: Color::Rgb(255, 145, 175),
    }
}

/// Void — ultra-dark with electric purple
pub fn void() -> Theme {
    Theme {
        name: "Void".to_string(),
        bg: Color::Rgb(8, 8, 16),
        fg: Color::Rgb(200, 200, 220),
        accent: Color::Rgb(155, 100, 255),     // electric purple
        accent_dim: Color::Rgb(110, 75, 180),
        gutter_bg: Color::Rgb(5, 5, 12),
        gutter_fg: Color::Rgb(50, 50, 75),
        active_line_bg: Color::Rgb(16, 16, 28),
        selection_bg: Color::Rgb(40, 30, 60),
        status_bg: Color::Rgb(5, 5, 12),
        status_fg: Color::Rgb(200, 200, 220),
        tab_bg: Color::Rgb(5, 5, 12),
        tab_active_bg: Color::Rgb(8, 8, 16),
        tab_fg: Color::Rgb(50, 50, 75),
        tab_active_fg: Color::Rgb(155, 100, 255),
        sidebar_bg: Color::Rgb(5, 5, 12),
        sidebar_fg: Color::Rgb(140, 140, 170),
        sidebar_active_bg: Color::Rgb(16, 16, 28),
        border: Color::Rgb(40, 30, 60),
        keyword: Color::Rgb(155, 100, 255),    // purple
        string: Color::Rgb(100, 255, 180),     // neon green
        comment: Color::Rgb(70, 70, 100),
        function: Color::Rgb(100, 180, 255),   // electric blue
        r#type: Color::Rgb(255, 150, 100),     // neon orange
        number: Color::Rgb(255, 100, 150),     // neon pink
        operator: Color::Rgb(180, 150, 255),
        error: Color::Rgb(255, 60, 80),
        warning: Color::Rgb(255, 200, 50),
        success: Color::Rgb(100, 255, 180),
        popup_bg: Color::Rgb(12, 12, 22),
        popup_border: Color::Rgb(155, 100, 255),
    }
}

/// Sea Software Dark — custom dark mode for Sea Software org
pub fn sea_software_dark() -> Theme {
    Theme {
        name: "Sea Software Dark".to_string(),
        bg: Color::Rgb(10, 14, 23),            // #0a0e17 (Background)
        fg: Color::Rgb(240, 244, 255),         // #f0f4ff (Text Primary)
        accent: Color::Rgb(59, 110, 248),      // #3b6ef8 (Accent From)
        accent_dim: Color::Rgb(0, 200, 255),   // #00c8ff (Accent To)
        gutter_bg: Color::Rgb(10, 14, 23),
        gutter_fg: Color::Rgb(75, 85, 99),     // #4b5563 (Text Muted)
        active_line_bg: Color::Rgb(17, 24, 39),// #111827 (Surface)
        selection_bg: Color::Rgb(30, 37, 53),  // #1e2535 (Elevated/Card)
        status_bg: Color::Rgb(17, 24, 39),
        status_fg: Color::Rgb(240, 244, 255),
        tab_bg: Color::Rgb(10, 14, 23),
        tab_active_bg: Color::Rgb(17, 24, 39),
        tab_fg: Color::Rgb(156, 163, 175),     // #9ca3af (Text Secondary)
        tab_active_fg: Color::Rgb(59, 110, 248),
        sidebar_bg: Color::Rgb(10, 14, 23),
        sidebar_fg: Color::Rgb(156, 163, 175),
        sidebar_active_bg: Color::Rgb(17, 24, 39),
        border: Color::Rgb(55, 65, 81),        // #374151 (Border)
        keyword: Color::Rgb(59, 110, 248),
        string: Color::Rgb(0, 200, 255),
        comment: Color::Rgb(156, 163, 175),
        function: Color::Rgb(59, 110, 248),
        r#type: Color::Rgb(0, 200, 255),
        number: Color::Rgb(0, 200, 255),
        operator: Color::Rgb(156, 163, 175),
        error: Color::Rgb(255, 100, 100),
        warning: Color::Rgb(255, 200, 80),
        success: Color::Rgb(0, 200, 255),
        popup_bg: Color::Rgb(30, 37, 53),
        popup_border: Color::Rgb(59, 110, 248), // #3b6ef8
    }
}

/// Sea Software Light — custom light mode for Sea Software org
pub fn sea_software_light() -> Theme {
    Theme {
        name: "Sea Software Light".to_string(),
        bg: Color::Rgb(240, 244, 255),         // #f0f4ff (Background)
        fg: Color::Rgb(13, 20, 36),            // #0d1424 (Text Primary)
        accent: Color::Rgb(59, 110, 248),      // #3b6ef8 (Accent From)
        accent_dim: Color::Rgb(0, 200, 255),   // #00c8ff (Accent To)
        gutter_bg: Color::Rgb(240, 244, 255),
        gutter_fg: Color::Rgb(156, 163, 196),  // #9ca3c4 (Text Muted)
        active_line_bg: Color::Rgb(255, 255, 255),// #ffffff (Surface)
        selection_bg: Color::Rgb(232, 237, 248),// #e8edf8 (Elevated/Card)
        status_bg: Color::Rgb(255, 255, 255),
        status_fg: Color::Rgb(13, 20, 36),
        tab_bg: Color::Rgb(240, 244, 255),
        tab_active_bg: Color::Rgb(255, 255, 255),
        tab_fg: Color::Rgb(74, 85, 120),       // #4a5578 (Text Secondary)
        tab_active_fg: Color::Rgb(59, 110, 248),
        sidebar_bg: Color::Rgb(240, 244, 255),
        sidebar_fg: Color::Rgb(74, 85, 120),
        sidebar_active_bg: Color::Rgb(255, 255, 255),
        border: Color::Rgb(209, 217, 240),     // #d1d9f0 (Border)
        keyword: Color::Rgb(59, 110, 248),
        string: Color::Rgb(0, 200, 255),
        comment: Color::Rgb(74, 85, 120),
        function: Color::Rgb(59, 110, 248),
        r#type: Color::Rgb(0, 200, 255),
        number: Color::Rgb(0, 200, 255),
        operator: Color::Rgb(74, 85, 120),
        error: Color::Rgb(200, 50, 60),
        warning: Color::Rgb(200, 140, 30),
        success: Color::Rgb(0, 200, 255),
        popup_bg: Color::Rgb(232, 237, 248),
        popup_border: Color::Rgb(59, 110, 248), // #3b6ef8
    }
}

/// Solarized — the classic Solarized Dark
pub fn solarized() -> Theme {
    Theme {
        name: "Solarized".to_string(),
        bg: Color::Rgb(0, 43, 54),             // #002b36
        fg: Color::Rgb(131, 148, 150),         // #839496
        accent: Color::Rgb(38, 139, 210),      // #268bd2 blue
        accent_dim: Color::Rgb(88, 110, 117),  // #586e75
        gutter_bg: Color::Rgb(0, 36, 46),
        gutter_fg: Color::Rgb(88, 110, 117),
        active_line_bg: Color::Rgb(7, 54, 66), // #073642
        selection_bg: Color::Rgb(7, 54, 66),
        status_bg: Color::Rgb(7, 54, 66),
        status_fg: Color::Rgb(147, 161, 161),
        tab_bg: Color::Rgb(0, 36, 46),
        tab_active_bg: Color::Rgb(0, 43, 54),
        tab_fg: Color::Rgb(88, 110, 117),
        tab_active_fg: Color::Rgb(147, 161, 161),
        sidebar_bg: Color::Rgb(0, 36, 46),
        sidebar_fg: Color::Rgb(131, 148, 150),
        sidebar_active_bg: Color::Rgb(7, 54, 66),
        border: Color::Rgb(7, 54, 66),
        keyword: Color::Rgb(133, 153, 0),      // #859900 green
        string: Color::Rgb(42, 161, 152),      // #2aa198 cyan
        comment: Color::Rgb(88, 110, 117),
        function: Color::Rgb(38, 139, 210),    // blue
        r#type: Color::Rgb(181, 137, 0),       // #b58900 yellow
        number: Color::Rgb(203, 75, 22),       // #cb4b16 orange
        operator: Color::Rgb(131, 148, 150),
        error: Color::Rgb(220, 50, 47),        // #dc322f
        warning: Color::Rgb(181, 137, 0),
        success: Color::Rgb(133, 153, 0),
        popup_bg: Color::Rgb(3, 48, 60),
        popup_border: Color::Rgb(38, 139, 210),
    }
}

/// Dracula — the beloved dark theme
pub fn dracula() -> Theme {
    Theme {
        name: "Dracula".to_string(),
        bg: Color::Rgb(40, 42, 54),            // #282a36
        fg: Color::Rgb(248, 248, 242),         // #f8f8f2
        accent: Color::Rgb(189, 147, 249),     // #bd93f9 purple
        accent_dim: Color::Rgb(130, 110, 180),
        gutter_bg: Color::Rgb(34, 36, 46),
        gutter_fg: Color::Rgb(98, 114, 164),   // #6272a4
        active_line_bg: Color::Rgb(68, 71, 90),// #44475a
        selection_bg: Color::Rgb(68, 71, 90),
        status_bg: Color::Rgb(68, 71, 90),
        status_fg: Color::Rgb(248, 248, 242),
        tab_bg: Color::Rgb(34, 36, 46),
        tab_active_bg: Color::Rgb(40, 42, 54),
        tab_fg: Color::Rgb(98, 114, 164),
        tab_active_fg: Color::Rgb(248, 248, 242),
        sidebar_bg: Color::Rgb(34, 36, 46),
        sidebar_fg: Color::Rgb(190, 190, 200),
        sidebar_active_bg: Color::Rgb(68, 71, 90),
        border: Color::Rgb(68, 71, 90),
        keyword: Color::Rgb(255, 121, 198),    // #ff79c6 pink
        string: Color::Rgb(241, 250, 140),     // #f1fa8c yellow
        comment: Color::Rgb(98, 114, 164),     // #6272a4
        function: Color::Rgb(80, 250, 123),    // #50fa7b green
        r#type: Color::Rgb(139, 233, 253),     // #8be9fd cyan
        number: Color::Rgb(189, 147, 249),     // purple
        operator: Color::Rgb(255, 121, 198),
        error: Color::Rgb(255, 85, 85),        // #ff5555
        warning: Color::Rgb(241, 250, 140),
        success: Color::Rgb(80, 250, 123),
        popup_bg: Color::Rgb(46, 48, 62),
        popup_border: Color::Rgb(189, 147, 249),
    }
}
