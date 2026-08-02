build:
    nix build .#spyglass

run:
    cargo run

try:
    ghostty --title=Spyglass -e ./result/bin/spyglass
