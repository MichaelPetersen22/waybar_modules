#!/bin/bash
arg="$1"

## Install Cargo
curl https://sh.rustup.rs -sSf | sh

if [ "$arg" == "yes" ]; then
 sudo pacman -Sy otf-font-awesome --needed --noconfirm
fi

source "$HOME/.cargo/env"

cargo install waybar_weather_display

echo -e "Waybar Weather Display Module Installed.
Run waybar_weather_display --help for options"