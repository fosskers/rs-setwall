# `setwall`

A simple script for automating the setting of a wallpaper.

It assumes you have `hsetroot` installed.

## Usage

```
Usage: setwall [OPTIONS]

Optional arguments:
  -h, --help

Available commands:
  set     Set a specific image file as the background.
  random  Choose a random image file from a given directory.
```

## Automation

If you have a custom `.xinitrc`, you can set this line:

```
/home/YOU/.cargo/bin/setwall random /home/YOU/backgrounds/
```

Assuming you have a directory full of images at that path.
Every time you log in, you'll have a different wallpaper!
