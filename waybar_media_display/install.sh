#!/bin/bash
## Install Cargo
curl https://sh.rustup.rs -sSf | sh
## Install playerctl
sudo pacman -S playerctl --noconfirm

source "$HOME/.cargo/env"

echo "Hello World"

## Install Module
cargo install waybar_media_display

echo -e "Waybar Media Display Module Installed.
Run waybar_media_display --help for options"
