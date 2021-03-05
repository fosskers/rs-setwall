# setwall

## 1.1.0 (2021-03-05)

#### Added

- `--comp`/`-c` option to specify the target compositor. Current legal values
  are `sway` and `x11`, with `x11` as the default.

#### Changed

- `-h` output is more useful.

## 1.0.3 (2020-07-18)

#### Changed

- Better release profile which produces smaller binaries.

## 1.0.2 (2020-06-25)

#### Fixed

- Images with uppercase file extensions are now recognized.

## 1.0.1 (2020-06-17)

#### Changed

- Dropped `structopt` for `gumdrop`. This reduced binary size by 20% and freed
  up many dependencies.
