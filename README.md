# `setwall`

A simple script for automating the setting of a wallpaper.

It assumes you have `hsetroot` installed.

## Usage

```
setwall 1.0.0

USAGE:
    setwall <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    random    Choose a random image file from a given directory
    set       Set a specific image file as the background
```

## Automation

If you have a custom `.xinitrc`, you can set this line:

```
/home/YOU/.cargo/bin/setwall random /home/YOU/backgrounds/
```

Assuming you have a directory full of images at that path.
Every time you log in, you'll have a different wallpaper!
