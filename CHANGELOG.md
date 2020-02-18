# Changelog


# tmux_interface vX.X.X


# tmux_interface v0.0.7
- implement builder pattern for structures of tmux subcommands
- add cargo features `tmux_2_6`, `tmux_X_X` for multiple tmux versions support
- add `TargetSession`, `TargetWindow`, `TargetWindowEx`,
    `TargetPane`, `TargetPaneEx` as an equivalent to `target-session`,
    `target-window`, `target-pane`
- restructuring of directories and files


# tmux_interface v0.0.6
- add `Layout` struct
- add `from_str` functions for parsing
- add `SessionStack` struct
- add `PaneTabs` struct
- add bitflags for parsing variables
- refactor arguments number and structures for functions
- move non-optional arguments out of structure
- add callback hooks (`pre_hook`, `post_hook`)
- add enum for pane size (size, percentage)
- accept `rustfmt`, `clippy` suggestions and formatting
- add inheritance of `stdin`, fix `open terminal failed: not a terminal`
- add `tmux 3.0a` support


# tmux_interface v0.0.5
- remove `regex` dependency
- add fsm for parsing


# tmux_interface v0.0.4
- add functions:
    - `show-options`
    - `exec`
    - `has-session`
    - `detach-client`
    - `kill-server`
    - `copy-mode`
    - `break-pane`
    - `capture-pane`
    - `choose-client`
    - `choose-tree`
    - `display-panes`
    - `find-window`
    - `kill-pane`
    - `join-pane`
    - `last-layout`
    - `next-layout`
    - `previous-layout`
    - `last-window`
    - `next-window`
    - `previous-window`
    - `rotate-window`
    - `swap-window`
    - `unlink-window`
    - `link-window`
    - `move-pane`
    - `move-window`
    - `resize-pane`
    - `resize-window`
    - `respawn-pane`
    - `respawn-window`
    - `pipe-pane`
    - `swap-pane`
    - `select-layout`

- add global and session environment functions
- add status line functions
- add buffers functions
- add miscellaneous functions
- add hooks functions
- add options functions
- add key bindings functions
- support `rust 1.37`

# tmux_interface v0.0.3
- remove `serde_yaml` dependency

# tmux_interface v0.0.2
- fix `regex` dependency

# tmux_interface v0.0.1
- add basic `tmux` functionality:
    - buffers
    - hooks
    - options
    - key bindings
    - windows and panes
    - clients and sessions
    - global and session environment
    - miscellaneous
    - status line
- add parsing variables
- add error propagating
