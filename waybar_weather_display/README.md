# Waybar Weather Display Module
![20220731_14h52m07s_grim-cropped](https://user-images.githubusercontent.com/72793125/182010979-70271606-3a8c-4c86-b7cb-1c5795949d58.png)
## Installation
With Pre-Requisites already installed

```
cargo install waybar_weather_display
```

Alternatively you can just run

```
curl https://raw.githubusercontent.com/MichaelPetersen22/waybar_modules/main/weather_install.sh -sSf | sh
```

If you want to also install the font used with the weather icons, run the below script

```
curl https://raw.githubusercontent.com/MichaelPetersen22/waybar_modules/main/weather_install.sh| bash -s -- yes
```

### Pre-Requisites
cargo

```
curl https://sh.rustup.rs -sSf | sh
```

otf-font-awesome

```
sudo pacman -S otf-font-awesome
```

## Usage
The command is not intended to be used on it's own as it prints back a json for waybar to read and convert into the module.

Example usage in waybar config is included below
```
    "custom/weather": {
        "interval": 900,
        "return-type":"json",
        "exec":"waybar_weather_display --latitude -27.4679 --longitude 153.0281",
	    "format-alt": "{} ",
	    "format":"{alt} ",
	    "format-alt-click":"click-right",
	    "escape": true
    }
```
Pay special attention to the "exec" field as that is where the module is called.

For details on how to use the command and the default values of the command, run ```waybar_weather_display --help```

## Planned
Configuration File Support

Additional CLI Parameters with Config File Support (i.e. Custom Icons, Custom Weather Condition Text)

Additional Return Data (I.e. get temperature, Humidity, Rain)

Rework to use nerd icons in place of Font Awesome, as font awesome does not have very many weather icons