use mlua::prelude::*;
use std::path::PathBuf;
use std::fs;

pub mod api;

pub struct PluginManager {
    lua: Lua,
}

impl PluginManager {
    pub fn new() -> LuaResult<Self> {
        let lua = Lua::new();
        Ok(Self { lua })
    }

    pub fn load_plugins(&mut self) -> LuaResult<()> {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let plugin_dir = home.join(".config/aether/plugins");

        if !plugin_dir.exists() {
            let _ = fs::create_dir_all(&plugin_dir);
            return Ok(());
        }

        for entry in fs::read_dir(plugin_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("lua") {
                let script = fs::read_to_string(&path)?;
                let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown");
                if let Err(e) = self.lua.load(&script).set_name(name).exec() {
                    eprintln!("Error loading plugin {}: {}", name, e);
                }
            }
        }

        Ok(())
    }

    pub fn run_hook(&self, hook_name: &str, app_ptr: *mut crate::app::App) -> LuaResult<()> {
        let globals = self.lua.globals();
        let old_ptr: usize = globals.get("__app_ptr")?;
        
        // Safety: Set the pointer to the current app instance
        globals.set("__app_ptr", app_ptr as usize)?;
        
        // If there's a global function with the hook name, call it
        if let Ok(func) = globals.get::<_, LuaFunction>(hook_name) {
            let _ : () = func.call(())?;
        }

        // Restore old pointer (usually 0)
        globals.set("__app_ptr", old_ptr)?;
        
        Ok(())
    }

    pub fn run_keypress_hook(&self, key: &str, app_ptr: *mut crate::app::App) -> LuaResult<bool> {
        let globals = self.lua.globals();
        let old_ptr: usize = globals.get("__app_ptr")?;
        globals.set("__app_ptr", app_ptr as usize)?;

        let mut handled = false;
        if let Ok(func) = globals.get::<_, LuaFunction>("on_keypress") {
            let res: bool = func.call(key)?;
            handled = res;
        }

        globals.set("__app_ptr", old_ptr)?;
        Ok(handled)
    }

    pub fn setup_api(&self) -> LuaResult<()> {
        crate::plugin::api::register_api(&self.lua)
    }
}
