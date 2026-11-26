use mlua::{Lua, LuaSerdeExt};
use std::{str::FromStr, fs};

pub fn primitives(state: &mut Lua) {
    let spyglass_table = state.create_table().unwrap();

    let toml_parser = state.create_function(|lua, input: String| {
        let table = toml::Table::from_str(&input)
            .map_err(mlua::Error::external)?;

        lua.to_value(&table)
    }).unwrap();
    let json_parser = state.create_function(|lua, input: String| {
        let value: serde_json::Value = serde_json::from_str(&input).map_err(mlua::Error::external)?;

        lua.to_value(&value)
    }).unwrap();
    let dir_files = state.create_function(|lua, dir: String| {
        let entries = fs::read_dir(&dir).map_err(mlua::Error::external).unwrap();

        let mut files = Vec::new();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            let kind = entry.file_type();

            if kind.as_ref().unwrap().is_file() || (kind.unwrap().is_symlink() && fs::metadata(&path).unwrap().is_file()) {
                files.push(path);
            }
        }

        lua.create_sequence_from(files)
    }).unwrap();

    spyglass_table.set("toml", toml_parser).unwrap();
    spyglass_table.set("json", json_parser).unwrap();
    spyglass_table.set("files", dir_files).unwrap();

    state.globals().set("spyglass", spyglass_table).unwrap();
}
