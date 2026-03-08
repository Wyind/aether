# Aether Theming Guide

Aether supports custom themes via TOML files. You can create your own color palettes and apply them instantly.

## Theme Location

Custom themes should be placed in `~/.config/aether/themes/`. Any `.toml` file in this directory will be loaded automatically and added to the theme cycle (F5) and command palette.

## Theme Format

A theme file is a simple TOML document defining various UI colors in Hex format (e.g., `#RRGGBB`).

### Required Fields
- `name`: The display name of your theme.
- `bg`: Background color.
- `fg`: Main text color.
- `accent`: Primary accent color (used for keywords, active elements).

### Optional Fields (Inherited or calculated if missing)
- `accent_dim`: Dimmed accent (default: darkened `accent`).
- `border`: UI border color (default: lightened `bg`).
- `gutter_fg`: Line number color (default: dimmed `fg`).
- `keyword`: Keyword color (default: `accent`).
- `string`: String literal color (default: lightened `accent`).
- `comment`: Comment color (default: `gutter_fg`).
- `function`: Function name color (default: lightened `accent`).
- `type`: Type definition color (default: `accent_dim`).
- `number`: Numeric literal color (default: lightened `accent`).
- `error`: Error/alert color (default: red).

## Example Theme: `ocean_depth.toml`

```toml
name = "Ocean Depth"
bg = "#0f172a"
fg = "#f8fafc"
accent = "#38bdf8"
accent_dim = "#0369a1"
border = "#1e293b"
keyword = "#38bdf8"
string = "#7dd3fc"
comment = "#475569"
function = "#0ea5e9"
```

## Tips
- Use high-contrast colors for `bg` and `fg` to ensure readability.
- The `accent_dim` color is often used for subtle UI elements like folder icons in the tree.
- Aether automatically calculates secondary colors (like `tab_bg` or `selection_bg`) based on your `bg` and `fg` choices for a consistent look.
