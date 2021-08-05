# Changelog


<!--## tmux_interface vX.X.X-->

## tmux_interface v0.2.0
- fix parsing of time in Session and Window (`session_activity`,
  `session_created`, `session_last_attached`, `window_activity`)

## tmux_interface v0.1.0
- introduce new architecture (massive incompatible changes with older versions)
- refactor tmux command wrapper functions
    - old simplified function call was:
      ```
      let mut tmux = TmuxInterface::new();
      tmux.new_session(NewSessionBuilder::new().session_name(...));

      // or
      let mut tmux = TmuxInterface::new();
      let new_session = NewSession {
        detached: Some(true),
        ..Default::default();
      };
      tmux.new_session(&new_session).unwrap();
      ```

    - change to simplified function call now:
      ```
      let mut tmux = TmuxCommand:new();
      tmux.new_session().detached().session_name(...)...(...).output().unwrap();

      // or directly
      NewSession::new().detached().session_name(...)...(...).output().unwrap();
      ```


## tmux_interface v0.0.7
- add cargo feature `use_cmd_alias` for alias support for tmux commands
  (`list-sessions` -> `ls`)
- implement builder pattern for structures of tmux subcommands
    (`NewSessionBuilder::new().session_name("my_session_name").build()`)
- add cargo features `tmux_0_8` ... `tmux_X_X` for multiple tmux versions support
- add target objects and corresponding functions (`TargetSession`,
`TargetWindow`, `TargetWindowExt`, `TargetPane`, `TargetPaneExt` as an equivalent
to `target-session`, `target-window`, `target-pane`)
- restructuring of directories and files add tmux options objects and functions
- add tmux options (`ServerOptions`, `SessionOptions`, `WindowOptions`, `PaneOptions`)
- add tmux variables (`session_*`, `window_*`, `pane_*`)


## tmux_interface v0.0.6
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


## tmux_interface v0.0.5
- remove `regex` dependency
- add fsm for parsing


## tmux_interface v0.0.4
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

## tmux_interface v0.0.3
- remove `serde_yaml` dependency

## tmux_interface v0.0.2
- fix `regex` dependency

## tmux_interface v0.0.1
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
