use crate::Lens;
use mlua::{Table, Lua};

pub fn init(lua: &Lua) -> Vec<Lens> {
    let builtin_lenses: Vec<String> = vec![
        #[cfg(feature = "apps")]
        r#"
local function loadConfigs()
  local dir = os.getenv("HOME") .. "/.config/spyglass/applications"
  local entries = spyglass.files(dir)
  local configs = {}

  for _, path in ipairs(entries) do
    local file = assert(io.open(path, "rb"))
    local content = file:read("*a")
    file:close()
    table.insert(configs, spyglass.toml(content))
  end

  table.sort(configs, function(a, b)
    return a.name:lower() < b.name:lower()
  end)
  return configs
end

return {
  name = "apps",
  search = function(query)
    local results = {}

    for _, config in ipairs(loadConfigs()) do
      local name = config.name
      if name and name:lower():find(query:lower(), 1, true) then
        table.insert(results, {
          id = name,
          title = name,
          description = config.description or "",
          icon = config.icon or "",
          meta = config.command or "",
          enter = config.command or "",
          alternate = config.command,
        })
      end
    end

    return results
  end,
}
        "#.to_string(),
];

    let mut lenses: Vec<Lens> = Vec::new();

    for i in builtin_lenses {
        let table: Table = lua
            .load(&i.clone())
            .set_name(i)
            .eval()
            .unwrap();

        lenses.push(Lens {
            name: table.get("name").unwrap(),
            search: table.get("search").unwrap(),
        })
    }

    lenses
}
