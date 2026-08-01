# Configuring the apps# lens

Configuration takes place in the `~/.config/spyglass/applications` directory.

To create an application entry, create a `.toml` file named after that entry.

```toml
name = "LibreOffice"                      # The name of the application
icon = ""                                # Icon, typically from a nerd font, or an emoji
command = "libreoffice & sleep 5"         # Command to run. Note that the `& sleep 5` is needed because sometimes the terminal will close before the program can be detached, and it won't launch
```
