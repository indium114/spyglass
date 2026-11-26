# Developing a Lens

Lenses are written in _Lua_ 5.5. they are stored in `~/.config/spyglass/lenses`. The builtin lenses can be disabled by compiling with `--no-default-features` and selecting the ones you want to keep.

To write a Lens, you should implement it as such:

```lua
return {
  name = "lens_name_here",
  search = function(query)
    local results = {}

    -- determine results here

    return results
  end,
}
```

To aid in development, a few primitives exist:

- `spyglass.toml(string) -> {}`: parses the input string as toml and returns a table
- `spyglass.json(string) -> {}`: same as toml but with json instead
- `spyglass.files(string) -> {}`: takes in a directory path and returns the list of _files_ in it
