# Configuring the web# lens

Configuration takes place in the `~/.config/spyglass/web/config.toml` file.

```toml
url = http://example.com/search?q=%s
# replace the above with the search url for whatever search engine you want to use.
# make sure to use '%s' where the query is supposed to go, spyglass will substitute your query into it.
```
