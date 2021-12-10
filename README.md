# `setwall`

A simple script for automating the setting of a wallpaper on Sway/Wayland or X11.

For X11, it assumes you have `hsetroot` installed.

## Usage

```
Usage: setwall [OPTIONS]

Optional arguments:
  -h, --help

Available commands:
  set     Set a specific image file as the background.
  random  Choose a random image file from a given directory.
```

- Specify `-c`/`--comp` to pick an output compositor. `-c sway` will use
  `swaybg` to replace your Sway background, while the default `-c x11` will use
  `hsetroot`.
- Specify `-o`/`--output` to pick a Sway output or X screen if using X11. If
  this flag is missing, the image will be applied to all screens.
- Specify `-k`/`--kill-swaybg` to kill existing swaybg processes.

For instance:

```sh
setwall random ~/Pictures -c sway -o DP-2 -o DP-3 -o HDMI-A-1
```

## X11 Automation

If you have a custom `.xinitrc`, you can set this line:

```
/home/YOU/.cargo/bin/setwall random /home/YOU/backgrounds/
```

Assuming you have a directory full of images at that path.
Every time you log in, you'll have a different wallpaper!
