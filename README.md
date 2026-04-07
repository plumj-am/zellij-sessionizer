# zellij-sessionizer

> [!NOTE]
> This plugin is a soft fork of
> [github:laperlej/zellij-sessionizer](https://github.com/laperlej/zellij-sessionizer)
> with a few improvements (and more coming):
>
> - More Vim keymaps by default
> - New option `show_hidden`
> - Clearer docs and installation guidance
> - Development Nix flake
> - ...

[showcase.webm](https://github.com/user-attachments/assets/dc1b3174-07ac-4210-a689-bdc2e16ee0de)

This plugin is based on ThePrimeagen's tmux sessionizer
[script](https://github.com/ThePrimeagen/.dotfiles/blob/master/bin/.local/scripts/tmux-sessionizer)

The idea is to provide a list of directories that contain your projects/repos.

When opened, the plugin displays a list of all the subdirectories (one level
deep) for you to select from.

When a directory is selected, a new session will be created with its name and
cwd set to the directory.

If the session already exists, it will attach instead.

The main difference from the built-in filepicker is that the search is done over
a single combined flat list, so there is no need to navigate the file system.

## Installation

There are 3 main ways:

1. Use the plugin via the URL below which has the benefit of staying updated.
   Only the first load will be slower, after that Zellij will cache the plugin
   locally.

```
https://github.com/plumj-am/zellij-sessionizer/releases/latest/download/zellij-sessionizer.wasm
```

2. Download zellij-sessionizer.wasm from the
   [latest release](https://github.com/plumj-am/zellij-sessionizer/releases/latest)
   and place it in your zellij plugins directory.

```bash
mkdir --parents ~/.config/zellij/plugins
wget https://github.com/plumj-am/zellij-sessionizer/releases/latest/download/zellij-sessionizer.wasm \
  --output-document ~/.config/zellij/plugins/zellij-sessionizer.wasm
```

3. Create a plugin alias.

```kdl
plugins {
  sessionizer location="https://github.com/plumj-am/zellij-sessionizer/releases/latest/download/zellij-sessionizer.wasm"
}
```

## Configuration

Add the plugin to a keybinding in your config.toml.

In this example, the keybinding is bound to `g` in tmux mode.

```kdl
tmux {
  bind "g" {
    LaunchOrFocusPlugin "https://github.com/plumj-am/zellij-sessionizer/releases/latest/download/zellij-sessionizer.wasm" {
    // LaunchOrFocusPlugin "file:~/.config/zellij/plugins/zellij-sessionizer.wasm" {
    // LaunchOrFocusPlugin "sessionizer" {
      floating true
      move_to_focused_tab true
      cwd "/"
      root_dirs "/home/plumjam/projects;/home/plumjam/workspaces"
      individual_dirs "/home/plumjam/.dotfiles;/etc/nginx"
      show_hidden ".config;.local"
      session_layout "myCustomLayout"
    }; SwitchToMode "Locked";
  }
}
```

Options:

- `cwd`: Should be set to `"/"` so that other path-related options work as
  expected.
- `root_dirs`: string of paths separated by a semicolon, default is `""`. Each
  path is scanned for subdirectories (1 level deep) which are then presented as
  selectable sessions.
- `individual_dirs`: string of paths separated by a semicolon, default is `""`.
  Each path is added directly as a selectable session, without scanning for
  subdirectories. This is useful for including specific directories that aren't
  grouped under a common parent.
- `show_hidden`: string of paths separated by a semicolon, default is
  `".config"`. Dot paths are hidden by default; this option lets you allow
  specific ones.
- `session_layout`: the layout to use for new sessions, please prepend the
  layout name with a `:` if you want to use a built-in layout e.g. `:compact`,
  default is `:default`. If there is a `layout.kdl` in the target directory it
  will be used instead.

**IMPORTANT:** Due to the way plugins interact with the filesystem the root_dirs
**must** be absolute paths and **must** be descendants of the `cwd` option.

## Usage

1. Spawn the picker
2. Search for and select a directory
3. Press `Enter` to open a new session or attach to the existing session in the
   directory

All actions:

- `Up`/`Down`, `Shift-Tab`/`Tab`, or `<C-p>`/`<C-n>`: Select previous/next
  directory.
- `<C-u>`/`<C-d>`: Move selection half a page up/down.
- `Enter`: Create a session based on selected directory.
- `Esc`/`<C-c>`: Close the picker.

## Contributing

Contributions are welcome. Please open an issue or a pull request.

## License

```
MIT License

Copyright (c) 2024-Present Jonathan Laperle
Copyright (c) 2025-present PlumJam <git@plumj.am>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
