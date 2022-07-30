#!/bin/bash
## Install Cargo
curl https://sh.rustup.rs -sSf | sh
## Install playerctl
sudo pacman -S playerctl

source "$HOME/.cargo/env"

## Install Module
cargo install waybar_media_display

echo -e "Waybar Media Display Module Installed.
Run waybar_media_display --help for options"
