use mlua::{
    Function,
    Lua,
    LuaSerdeExt,
    Table
}

pub struct Lens {
    name: String,
    search: Function,
}

#[derive(Clone)]
pub struct Entry {
    /// unique internal id for each entry
    pub id: String,
    /// user-facing, 'pretty' title
    pub title: String,
    /// entry description
    pub description: String,
    /// single-character icon for the entry
    pub icon: String,
    /// extra info, typically used if `enter()` needs more info
    pub meta: String,
    /// function to run when the user presses _Enter_ on the entry
    pub enter: fn(&Entry),
    /// alternate function to run when the user presses _Tab_ on the entry
    pub alternate: Option<fn(&Entry)>,
}

pub fn load_lenses(lua: &Lua, dir: &str) -> Vec<Lens> {
    let mut lenses: Vec<Lens = Vec::new();

    for item in fs::read_dir(dir).unwrap() {
        let path = item.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("lua") {
            continue;
        }

        let src = fs::read_to_string(&path).unwrap();
        let table: Table = lua
            .load(&src)
            .set_name(path.to_string_lossy())
            .eval()
            .unwrap();

        lenses.push(
            Lens {
                name: table.get("name").unwrap(),
                search: table.get("search").unwrap(),
            }
        )
    }

    lenses
}
