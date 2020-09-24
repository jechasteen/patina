# Patina Window Manager

Patina is a work in progress.

See [milestones](https://github.com/jechasteen/patina/milestones)

***

## What it does right now

Version 0.1 is literally just connecting to X and showing an xterm window.

## Dependencies

### Ubuntu

* libglib2.0-dev
* libcairo2-dev

## Building

You can build and run an instance with a single command

```sh
./test debug
# or
./test release
```

The script will handle building either if they haven't been built yet.
It does not handle cleaning yet, so just issue `cargo clean` directly.

## Design goals

1. Async
2. Small code base, but not sloc limited
3. Customization via patches, a la dwm

## Features

1. Tiling and floating windows, toggle between
2. Panel (like dwm but with more features out of the box)
3. Launcher, something like dmenu, rofi, albert, etc.
4. Workspaces
5. Support multi-head systems
6. Configuration file (toml)
    - Colors
    - Keybinds
    - Workspace quantity and names
    - Select from available layouts
7. Change tiling arrangement via keyboard or mouse.
