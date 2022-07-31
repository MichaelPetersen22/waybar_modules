# Waybar Media Display Module
![Media Player Module](https://user-images.githubusercontent.com/72793125/181866336-9ccfdc0e-7f72-4408-8414-604528eea305.png)

## Installation
With Pre-Requisites already installed

```
cargo install waybar_media_display
```
Alternatively you can just run

```
curl https://raw.githubusercontent.com/MichaelPetersen22/waybar_modules/main/media_install.sh -sSf | sh
```

If you want to also install the font used with the default icon option (The spotify icon), run the below script

```
curl https://raw.githubusercontent.com/MichaelPetersen22/waybar_modules/main/media_install.sh| bash -s -- yes
```

### Pre-Requisites
cargo

```
curl https://sh.rustup.rs -sSf | sh
```

playerctl

```
sudo pacman -S playerctl
```

otf-font-awesome

```
sudo pacman -S otf-font-awesome
```

## Usage
The command is not intended to be used on it's own as it prints back a json for waybar to read and convert into the module.

Example usage in waybar config is included below
```
 "custom/spotify": {
 
       "interval": 1,
       
       "return-type": "json",
       
       "exec": "waybar_media_display",
       
       "exec-if": "pgrep spotify",
       
       "on-click":"playerctl --player=spotify play-pause",
 
       "escape": true
       
    }
```
Pay special attention to the "exec" field as that is where the module is called.

The example utilizes the defaults of the media display, however you can provide it with any media player supported by playerctl, and you can provide it any text or font that you wish for the icon. 
For my spotify icon I used ```otf-font-awesome```

For details on how to use the command and the default values of the command, run ```waybar_media_display --help```
