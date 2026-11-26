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
        #[cfg(feature = "power")]
        r#"
return {
  name = "power",
  search = function(query)
    local results = {
      {
        id = "poweroff",
        title = "Shutdown",
        description = "",
        icon = "⏻",
        meta = "",
        enter = "systemctl poweroff",
        alternate = "",
      },
      {
        id = "reboot",
        title = "Reboot",
        description = "",
        icon = "",
        meta = "",
        enter = "systemctl reboot",
        alternate = "",
      },
      {
        id = "suspend",
        title = "Suspend",
        description = "",
        icon = "⏾",
        meta = "",
        enter = "systemctl suspend",
        alternate = "",
      },
    }

    local finalResults = {}

    for _, e in ipairs(results) do
      if e.title and e.title:lower():find(query:lower(), 1, true) then
        table.insert(finalResults, entry)
      end
    end

    return finalResults
  end,
}
        "#.to_string(),
        #[cfg(feature = "process")]
        r#"
local listProcesses = function()
  local pipe = assert(io.popen("ps -e -o pid= -o comm=", "r"))
  local processes = {}

  for line in pipe:lines() do
      local pid, name = line:match("^%s*(%d+)%s+(.+)$")
      if pid and name then
          processes[#processes + 1] = {
              pid = pid,
              name = name,
          }
      end
  end

  pipe:close()
  return processes
end

return {
  name = "process",
  search = function(query)
    local results = {}

    for _, process in ipairs(listProcesses()) do
      if process.name and process.name:lower():find(query:lower(), 1, true) then
        table.insert(results, {
          id = process.pid,
          title = process.pid .. " " .. process.name,
          description = "",
          icon = "",
          meta = "",
          enter = "kill " .. process.pid,
          alternate = "",
        })
      end
    end

    return results
  end
}
        "#.to_string(),
        #[cfg(feature = "web")]
        r#"
local loadConfig = function()
  local path = os.getenv("HOME") .. "/.config/spyglass/web/config.toml"
  local file = assert(io.open(path, "rb"))
  local content = file:read("*a")
  file:close()

  return spyglass.toml(content)
end

local urlEncode = function(s)
  return (s:gsub("([^A-Za-z0-9%-._~])", function(c)
    return string.format("%%%02X", string.byte(c))
  end))
end

return {
  name = "web",
  search = function(query)
    return {{
      id = "web",
      title = query,
      description = "",
      icon = "",
      meta = "",
      enter = "xdg-open " .. loadConfig().url:gsub("%s", urlEncode(query)),
      alternate = "",
    }}
  end,
}
        "#.to_string(),
];

    let mut lenses: Vec<Lens> = Vec::new();

    for i in builtin_lenses {
        let table: Table = lua
            .load(i.clone())
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
