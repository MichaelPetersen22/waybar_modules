#!/bin/bash
## Install Cargo
curl https://sh.rustup.rs -sSf --noconfirm | sh
## Install playerctl
sudo pacman -S playerctl --needed --noconfirm
echo "Hello World"

source "$HOME/.cargo/env"

## Install Module
cargo install waybar_media_display

echo -e "Waybar Media Display Module Installed.
Run waybar_media_display --help for options"
