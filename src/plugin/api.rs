use mlua::prelude::*;
use crate::app::App;

pub fn register_api(lua: &Lua) -> LuaResult<()> {
    let aether = lua.create_table()?;

    // Status message
    aether.set("set_status", lua.create_function(|lua, msg: String| {
        let globals = lua.globals();
        let app_ptr: usize = globals.get("__app_ptr")?;
        if app_ptr == 0 { return Ok(()); }
        unsafe {
            let app = &mut *(app_ptr as *mut App);
            app.set_status(&msg);
        }
        Ok(())
    })?)?;

    // Get current line
    aether.set("get_line", lua.create_function(|lua, row: Option<usize>| {
        let globals = lua.globals();
        let app_ptr: usize = globals.get("__app_ptr")?;
        if app_ptr == 0 { return Ok(String::new()); }
        unsafe {
            let app = &*(app_ptr as *const App);
            if app.documents.is_empty() { return Ok(String::new()); }
            let doc = &app.documents[app.active_tab];
            let r = row.unwrap_or(doc.cursor.row);
            if r < doc.buffer.line_count() {
                Ok(doc.buffer.get_line(r).to_string())
            } else {
                Ok(String::new())
            }
        }
    })?)?;

    // Insert text
    aether.set("insert_text", lua.create_function(|lua, text: String| {
        let globals = lua.globals();
        let app_ptr: usize = globals.get("__app_ptr")?;
        if app_ptr == 0 { return Ok(()); }
        unsafe {
            let app = &mut *(app_ptr as *mut App);
            if !app.documents.is_empty() {
                let doc = &mut app.documents[app.active_tab];
                for c in text.chars() {
                    doc.insert_char(c);
                }
            }
        }
        Ok(())
    })?)?;

    // Get cursor position
    aether.set("get_cursor", lua.create_function(|lua, _: ()| {
        let globals = lua.globals();
        let app_ptr: usize = globals.get("__app_ptr")?;
        if app_ptr == 0 { return Ok((0, 0)); }
        unsafe {
            let app = &*(app_ptr as *const App);
            if app.documents.is_empty() { return Ok((0, 0)); }
            let doc = &app.documents[app.active_tab];
            Ok((doc.cursor.row, doc.cursor.col))
        }
    })?)?;

    // Save current file
    aether.set("save", lua.create_function(|lua, _: ()| {
        let globals = lua.globals();
        let app_ptr: usize = globals.get("__app_ptr")?;
        if app_ptr == 0 { return Ok(()); }
        unsafe {
            let app = &mut *(app_ptr as *mut App);
            app.save_current();
        }
        Ok(())
    })?)?;

    // Switch tab
    aether.set("switch_tab", lua.create_function(|lua, index: usize| {
        let globals = lua.globals();
        let app_ptr: usize = globals.get("__app_ptr")?;
        if app_ptr == 0 { return Ok(()); }
        unsafe {
            let app = &mut *(app_ptr as *mut App);
            if index < app.documents.len() {
                app.active_tab = index;
            }
        }
        Ok(())
    })?)?;

    // Get tab count
    aether.set("get_tab_count", lua.create_function(|lua, _: ()| {
        let globals = lua.globals();
        let app_ptr: usize = globals.get("__app_ptr")?;
        if app_ptr == 0 { return Ok(0usize); }
        unsafe {
            let app = &*(app_ptr as *const App);
            Ok(app.documents.len())
        }
    })?)?;

    // High-precision time in seconds
    aether.set("get_time", lua.create_function(|_, _: ()| {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).unwrap_or_default();
        Ok(duration.as_secs_f64())
    })?)?;

    lua.globals().set("aether", aether)?;
    lua.globals().set("__app_ptr", 0usize)?;

    Ok(())
}
