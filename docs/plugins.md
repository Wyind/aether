# Aether Plugin Guide

Aether features a powerful Lua plugin system that allows you to extend the editor's functionality, handle keypresses, and automate tasks.

## Getting Started

Plugins are loaded from `~/.config/aether/plugins/` on startup. Any file ending in `.lua` in this directory will be executed.

## API Reference

The global `aether` table provides access to the editor's core functions.

### `aether.set_status(message: string)`
Sets the message displayed in the status bar.

### `aether.get_line(row: number?) -> string`
Gets the content of the specified line (0-indexed). If no row is provided, returns the current line.

### `aether.insert_text(text: string)`
Inserts text at the current cursor position.

### `aether.get_cursor() -> table`
Returns a table with `row` and `col` fields.

### `aether.save()`
Saves the current document.

### `aether.switch_tab(index: number)`
Switches to the tab at the given index (0-indexed).

### `aether.get_tab_count() -> number`
Returns the total number of open tabs.

## Hooks

Define these global functions in your scripts to react to editor events.

### `on_keypress(key: string) -> boolean`
Called whenever a key is pressed. 
- `key`: A string representation of the key code (e.g., `"Char('a')"`, `"Char('s')"`, `"Enter"`, `"Esc"`).
- **Return**: `true` if the plugin handled the key (prevents default behavior), `false` otherwise.

### `on_save()`
Called just before a document is saved.

## Examples

### Key Blocker
```lua
-- Prevents typing the letter 'j'
function on_keypress(key)
    if key == "Char('j')" then
        aether.set_status("No 'j' allowed!")
        return true
    end
    return false
end
```

### Save Notification
```lua
function on_save()
    local cursor = aether.get_cursor()
    aether.set_status("Saved at line " .. (cursor.row + 1))
end
```
