use crate::Lens;
use mlua::{Table, Lua};

pub fn init(lua: &Lua) -> Vec<Lens> {
    let builtin_lenses: Vec<String> = vec![
        #[cfg(feature = "apps")]
        r#"
local loadConfigs = function()
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
        #[cfg(feature = "calc")]
        r#"
local eval_number = function(expr)
  local env = {
    sqrt = math.sqrt,
    cbrt = function(x)
      return x < 0 and -((-x) ^ (1 / 3)) or x ^ (1 / 3)
    end
  }
  local chunk = load("return " .. expr, "expression", "t", env)
  if not chunk then
      return "NaN"
  end

  local ok, result = pcall(chunk)
  if not ok or type(result) ~= "number" then
      return "NaN"
  end

  return result
end

return {
  name = "calc",
  search = function(query)
    local result, err = eval_number(query)
    result = tostring(result)
    if result then
      return {{
        id = result or "",
        title = tostring(result) or "",
        description = "",
        icon = "",
        meta = result,
        enter = "wl-copy " .. result,
        alternate = "",
      }}
    else
      return {{
        id = "err",
        title = tostring(err) or "",
        description = "Error during calculation",
        icon = "",
        meta = err or "",
        enter = "",
        alternate = "",
      }}
    end
  end,
}
        "#.to_string(),
        #[cfg(feature = "clipboard")]
        r#"
local cliphist = function()
  local pipe = assert(io.popen("cliphist list", "r"))
  local lines_raw = {}

  for line in pipe:lines() do
    lines_raw[#lines_raw + 1] = line
  end

  local lines = {}
  for _, line in ipairs(lines_raw) do
      local key, value = line:match("^%s*(%S+)%s+(.+)%s*$")
      if key and value then
          lines[key] = value
      end
  end

  pipe:close()
  return lines
end

return {
  name = "clipboard",
  search = function(query)
    local results = {}

    for i, e in pairs(cliphist()) do
      if e and e:lower():find(query:lower(), 1, true) then
        table.insert(results, {
          id = i,
          title = e,
          description = "",
          icon = "",
          meta = "",
          enter = "wl-copy " .. e,
          alternate = "cliphist wipe",
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
