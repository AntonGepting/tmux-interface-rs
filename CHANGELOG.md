# Changelog

<!--## tmux_interface vX.X.X-->

## tmux_interface v0.3.1
* feature: add `StdIO` enum and `.stdin()` `.stdout()` `.stderr()` methods of
  `Tmux`, allowing user controlled i/o redirection (e.g. silencing `.stderr()`)
* fix(#15): fix `.shell_command()` method of `SplitWindow`

## tmux_interface v0.3.0
Massive changes incompatible with latest release.
Command building and combining mechanisms changed.
Still can be used very carefully on your own risk.
Still missing almost all library documentation
(principles of usage can be partially derived from unit tests or integration tests).

- Refactor: **breaking changes** options support (`options` module)
Command building mechanisms instead of bitflags
* Getter/Setter structures for building getter/setter option commands
* Single/Multiple/All getter/setter for options
* Local/Global options support
* Controller structures for working with all options structures
  Submodules:
    * `ServerOptions`
    * `SessionOptions`
      * `GlobalSessionOptions`
      * `LocalSessionOptions`
    * `WindowOptions`
      * `GlobalWindowOptions`
      * `LocalWindowOptions`
    * `PaneOptions`
    * `UserOptions`

  Control (getter/setter):
    * `ServerOptionsCtl`
    * `SessionOptionsCtl`
      * `GlobalSessionOptionsCtl`
      * `LocalSessionOptionsCtl`
    * `WindowOptionsCtl`
      * `GlobalWindowOptionsCtl`
      * `LocalWindowOptionsCtl`
    * `PaneOptionsCtl`

- Feature: add macro feature for commands builder using lazy short flags instead of looking for full methods names (e.g. `new_session!(-s "session_name")`, `kill_session!(-t "session_name)` ... ).

- Refactor/Feature: variables (`variables` module), change from bitflags to format enums
  Submodules:
  * `Sessions`
  * `Windows`
  * `Panes`
  Controller (getter):
  * `SessionsCtl`
  * `WindowsCtl`
  * `PanesCtl`

- Feature: `TmuxCommand`, `cmd_builder`

- Feature: add styles partial support (`styles` module)
(not fully functional yet, preparing for review, contradictions, compatibility with other modules and future development, etc)
  * `Align`
  * `Colour`
  * `Range`
  * `StyleList`
  * `Style`

- Feature: add formats partial support (`formats` module). Used mainly for variables module. Allows to build custom format strings for requesting custom tmux variables lists and parsing later in structures.
  * `Variable` - single Variable as enum with fields for tmux variables (as string e.g. `{#window_active}`)
  * `Formats` - list with variables and separator (as string e.g. `{#window_active}'{#window_index}'{#...}`)
  * `VariableOutput` - single pointer to variable or structure field where output should be stored ()
  * `FormatsOutput` - list with pointers to variables or structure fields where output should be stored ()

- Feature: add tmux control mode support draft (`control_mode` module)
(not fully functional yet, preparing for review, contradictions, compatibility with other modules and future development, etc)

- Feature: add tmux 3.3 partial support (cargo features: `tmux_3_3`, `tmux_3_3a`)
- Feature: add tmux 3.2 partial support (cargo features: `tmux_3_2`, `tmux_3_2a`)
- Feature: add tmux 3.1 partial support (cargo features: `tmux_3_1`, `tmux_3_1a`, `tmux_3_1b`, `tmux_3_1c`)

- Feature(#11): **breaking changes**, switch back to builder pattern for tmux
  commands.

- **breaking changes**, switch from borrowing to owning builder pattern

- commands are now made more compatible with default and control modes of tmux,
  binary executable part of command moved to separate struct

- order of given arguments for commands doesn't matter, `.build()` function uses
  default order of arguments from tmux manual

- fix(#8): fix `ShowOptions` arguments order

- fix(#10): fix typing mistake in structure name, `SelectLayout` struct of
  `.select_layout()` command

- fix(#9): fix typing mistake in function name, `.background()` method of
  `.run_shell()` and `if_shell` commands

- refactor: restructuring of modules, submodules, directories, files

- fix: adapt/rework/rewrite integration and unit tests

## tmux_interface v0.2.1
- fix #8 set globality before option name in `SessionOption::get_global()`

## tmux_interface v0.2.0
- fix #5 parsing of time in Session and Window (`session_activity`,
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
    - rename crate feature `use_cmd_alias` -> `cmd_alias`

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
