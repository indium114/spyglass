use mlua::{Lua, LuaSerdeExt};
use std::str::FromStr;

pub fn primitives(state: &mut Lua) {
    let spyglass_table = state.create_table().unwrap();

    let toml_parser = state.create_function(|lua, input: String| {
        let value = toml::Value::from_str(&input)
            .map_err(mlua::Error::external).unwrap();

        lua.to_value(&value)
    }).unwrap();

    spyglass_table.set("toml", toml_parser).unwrap();

    state.globals().set("spyglass", spyglass_table).unwrap();
}
