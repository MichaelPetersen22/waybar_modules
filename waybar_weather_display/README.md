# Waybar Weather Display Module

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
 
       "interval": 1800,
       
       "return-type": "json",
       
       "exec": "waybar_weather_display --latitude 51.5085 --longitude -0.1257",
 
       "escape": true
    }
```
Pay special attention to the "exec" field as that is where the module is called.

For details on how to use the command and the default values of the command, run ```waybar_weather_display --help```

## Planned
I plan on adding support for configuration files wherein you can specify all of the command line params (lat, long) as well as customize what icons are used, text for the weather conditions and measurement type (i.e. celsius or fahrenheight). I may also add in support for being able to ask for extra information from the API (i.e. the ability to get temperature, humidity, rain amount) however this may be quite ambitious as it will require the ability to build out URL params for the API request.