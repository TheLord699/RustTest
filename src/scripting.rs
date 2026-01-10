use mlua::{Lua, Result, Table};

pub struct LuaAI {
    lua: Lua,
}

impl LuaAI {
    pub fn new() -> Self {
        Self {
            lua: Lua::new(),
        }
    }

    pub fn load_enemy_ai(&self, path: &str) -> Result<Table> {
        let code = std::fs::read_to_string(path)?;
        let ai: Table = self.lua.load(&code).eval()?;
        Ok(ai)
    }
}
