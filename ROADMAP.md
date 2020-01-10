# Roadmap

Some goals for further versions and current development are listed below.
Please send an [e-mail](mailto:anton.gepting@gmail.com) or open an
[issue](https://github.com/AntonGepting/tmux-interface-rs/issues/new)
if any feature is missing or if you have a request, an improvment, an idea etc.


**tmux interface v1.0.0**

- [ ] Support all tmux subcommands
    - [x] Clients and Sessions
    - [x] Windows and Panes
    - [x] Key Bindings
    - [x] Options
    - [ ] Support all tmux options
    - [x] Hooks
    - [x] Global and Session Environments
    - [x] Status Line
    - [x] Buffers
    - [x] Miscellaneous
- [ ] Documentation
    - [ ] Clients and Sessions
    - [ ] Windows and Panes
    - [ ] Key Bindings
    - [ ] Options
    - [ ] Support all tmux options
    - [ ] Hooks
    - [ ] Global and Session Environments
    - [ ] Status Line
    - [ ] Buffers
    - [ ] Miscellaneous
- [ ] Tests
    - [ ] Clients and Sessions
    - [ ] Windows and Panes
    - [ ] Key Bindings
    - [ ] Options
    - [ ] Support all tmux options
    - [ ] Hooks
    - [ ] Global and Session Environments
    - [ ] Status Line
    - [ ] Buffers
    - [ ] Miscellaneous
- [ ] Improve output, return
    - [ ] Clients and Sessions
    - [ ] Windows and Panes
    - [ ] Key Bindings
    - [ ] Options
    - [ ] Support all tmux options
    - [ ] Hooks
    - [ ] Global and Session Environments
    - [ ] Status Line
    - [ ] Buffers
    - [ ] Miscellaneous
- [ ] Freeze basic architecture
- [ ] Freeze API
- [ ] Add Travis CI all tmux releases test


**tmux interface v0.1.0**

- [ ] Support basic tmux subcommands (references:
[tmuxinator](https://github.com/tmuxinator/tmuxinator),
[libtmux](https://github.com/tmux-python/libtmux), rust crates using tmux)
- [ ] Parse full information from tmux responses into structures
    - [ ] Session
    - [ ] Window
    - [ ] Pane
    - [ ] Layout
    - [ ] Options

Parsing objects and supported tmux variables:

```
    Object     Variable name       Alias Replaced with
===============================================================================================
[ ] Pane       alternate_on              1 if pane is in alternate screen
[ ] ?          alternate_saved_x         Saved cursor X in alternate screen
[ ] ?          alternate_saved_y         Saved cursor Y in alternate screen
[ ] Buffer     buffer_created            Time buffer created
[ ] Buffer     buffer_name               Name of buffer
[ ] Buffer     buffer_sample             Sample of start of buffer
[ ] Buffer     buffer_size               Size of the specified buffer in bytes
[ ] Client     client_activity           Time client last had activity
[ ] Client     client_control_mode       1 if client is in control mode
[ ] Client     client_created            Time client created
[ ] Client     client_discarded          Bytes discarded when client behind
[ ] Client     client_height             Height of client
[ ] Client     client_key_table          Current key table
[ ] Client     client_last_session       Name of the client's last session
[ ] Client     client_name               Name of client
[ ] Client     client_pid                PID of client process
[ ] Client     client_prefix             1 if prefix key has been pressed
[ ] Client     client_readonly           1 if client is readonly
[ ] Client     client_session            Name of the client's session
[ ] Client     client_termname           Terminal name of client
[ ] Client     client_termtype           Terminal type of client
[ ] Client     client_tty                Pseudo terminal of client
[ ] Client     client_utf8               1 if client supports utf8
[ ] Client     client_width              Width of client
[ ] Client     client_written            Bytes written to client
[ ] Command    command                   Name of command in use, if any
[ ] Command    command_list_alias        Command alias if listing commands
[ ] Command    command_list_name         Command name if listing commands
[ ] Command    command_list_usage        Command usage if listing commands
[ ] Cursor     cursor_character          Character at cursor in pane
[ ] Cursor     cursor_flag               Pane cursor flag
[ ] Cursor     cursor_x                  Cursor X position in pane
[ ] Cursor     cursor_y                  Cursor Y position in pane
[ ] History    history_bytes             Number of bytes in window history
[ ] History    history_limit             Maximum window history lines
[ ] History    history_size              Size of history in lines
[ ] Hook       hook                      Name of running hook, if any
[ ] Hook       hook_pane                 ID of pane where hook was run, if any
[ ] Hook       hook_session              ID of session where hook was run, if any
[ ] Hook       hook_session_name         Name of session where hook was run, if any
[ ] Hook       hook_window               ID of window where hook was run, if any
[ ] Hook       hook_window_name          Name of window where hook was run, if any
[ ] ?          host                   #H Hostname of local host
[ ] ?          host_short             #h Hostname of local host (no domain name)
[ ] Pane       insert_flag               Pane insert flag
[ ] Pane       keypad_cursor_flag        Pane keypad cursor flag
[ ] Pane       keypad_flag               Pane keypad flag
[ ] ?          line                      Line number in the list
[ ] Mouse      mouse_all_flag            Pane mouse all flag
[ ] Mouse      mouse_any_flag            Pane mouse any flag
[ ] Mouse      mouse_button_flag         Pane mouse button flag
[ ] Mouse      mouse_line                Line under mouse, if any
[ ] Mouse      mouse_sgr_flag            Pane mouse SGR flag
[ ] Mouse      mouse_standard_flag       Pane mouse standard flag
[ ] Mouse      mouse_utf8_flag           Pane mouse UTF-8 flag
[ ] Mouse      mouse_word                Word under mouse, if any
[ ] Mouse      mouse_x                   Mouse X position, if any
[ ] Mouse      mouse_y                   Mouse Y position, if any
[ ] Pane       origin_flag               Pane origin flag
[+] Pane       pane_active               1 if active pane
[+] Pane       pane_at_bottom            1 if pane is at the bottom of window
[+] Pane       pane_at_left              1 if pane is at the left of window
[+] Pane       pane_at_right             1 if pane is at the right of window
[+] Pane       pane_at_top               1 if pane is at the top of window
[+] Pane       pane_bottom               Bottom of pane
[+] Pane       pane_current_command      Current command if available
[+] Pane       pane_current_path         Current path if available
[+] Pane       pane_dead                 1 if pane is dead
[+] Pane       pane_dead_status          Exit status of process in dead pane
[+] Pane       pane_format               1 if format is for a pane (not assuming the current)
[+] Pane       pane_height               Height of pane
[+] Pane       pane_id                #D Unique pane ID
[+] Pane       pane_in_mode              1 if pane is in a mode
[+] Pane       pane_index             #P Index of pane
[+] Pane       pane_input_off            1 if input to pane is disabled
[+] Pane       pane_left                 Left of pane
[+] Pane       pane_marked               1 if this is the marked pane
[+] Pane       pane_marked_set           1 if a marked pane is set
[+] Pane       pane_mode                 Name of pane mode, if any
[+] Pane       pane_pid                  PID of first process in pane
[+] Pane       pane_pipe                 1 if pane is being piped
[+] Pane       pane_right                Right of pane
[+] Pane       pane_search_string        Last search string in copy mode
[+] Pane       pane_start_command        Command pane started with
[+] Pane       pane_synchronized         1 if pane is synchronized
[+] Pane       pane_tabs                 Pane tab positions
[+] Pane       pane_title             #T Title of pane
[+] Pane       pane_top                  Top of pane
[+] Pane       pane_tty                  Pseudo terminal of pane
[+] Pane       pane_width                Width of pane
[ ] Server     pid                       Server PID
[ ] ?          rectangle_toggle          1 if rectangle selection is activated
[ ] ?          scroll_position           Scroll position in copy mode
[ ] ?          scroll_region_lower       Bottom of scroll region in pane
[ ] ?          scroll_region_upper       Top of scroll region in pane
[ ] ?          selection_present         1 if selection started in copy mode
[+] Session    session_activity          Time of session last activity
[+] Session    session_alerts            List of window indexes with alerts
[+] Session    session_attached          Number of clients session is attached to
[+] Session    session_created           Time session created
[+] Session    session_format            1 if format is for a session (not assuming the current)
[+] Session    session_group             Name of session group
[+] Session    session_group_list        List of sessions in group
[+] Session    session_group_size        Size of session group
[+] Session    session_grouped           1 if session in a group
[+] Session    session_id                Unique session ID
[+] Session    session_last_attached     Time session last attached
[+] Session    session_many_attached     1 if multiple clients attached
[+] Session    session_name           #S Name of session
[+] Session    session_stack             Window indexes in most recent order
[+] Session    session_windows           Number of windows in session
[ ] Server     socket_path               Server socket path
[ ] Server     start_time                Server start time
[ ] Server     version                   Server version
[+] Window     window_active             1 if window active
[+] Window     window_activity           Time of window last activity
[+] Window     window_activity_flag      1 if window has activity
[+] Window     window_bell_flag          1 if window has bell
[+] Window     window_bigger             1 if window is larger than client
[+] Window     window_end_flag           1 if window has the highest index
[+] Window     window_flags           #F Window flags
[+] Window     window_format             1 if format is for a window (not assuming the current)
[+] Window     window_height             Height of window
[+] Window     window_id                 Unique window ID
[+] Window     window_index           #I Index of window
[+] Window     window_last_flag          1 if window is the last used
[+] Window     window_layout             Window layout description, ignoring zoomed window panes
[+] Window     window_linked             1 if window is linked across sessions
[+] Window     window_name            #W Name of window
[+] Window     window_offset_x           X offset into window if larger than client
[+] Window     window_offset_y           Y offset into window if larger than client
[+] Window     window_panes              Number of panes in window
[+] Window     window_silence_flag       1 if window has silence alert
[+] Window     window_stack_index        Index in session most recent stack
[+] Window     window_start_flag         1 if window has the lowest index
[+] Window     window_visible_layout     Window layout description, respecting zoomed window panes
[+] Window     window_width              Width of window
[+] Window     window_zoomed_flag        1 if window is zoomed
[ ] Pane       wrap_flag                 Pane wrap flag
 ```
- [ ] Parsed structures check Type
    - [ ] Session
    - [ ] Window
    - [ ] Pane
- [ ] Parsed structures check Option
    - [ ] Session
    - [ ] Window
    - [ ] Pane
- [ ] Parse all tmux variables
- [ ] Prepare documentation
- [ ] Prepare tests
- [ ] Function results and errors


**tmux interface v0.0.6**

- [ ] Documentation for all existing functionality and items
- [ ] All tmux functions output return in right way
- [ ] No panics, no unwrap in lib functions
- [ ] Error reporting information num, enum, string like in std
- [ ] Better names for tmux subcommands wrapper function arguments
- [x] Check new `tmux` version (`tmux 3.0a`) for significant changes


**tmux interface v0.0.1**

- [x] Prepare sources for publication on github.com
    - [x] .editorconfig
    - [x] .travis.yml
    - [x] LICENSE.md
    - [x] README.md
    - [x] ROADMAP.md
    - [x] CHANGELOG.md
- [x] Prepare crate for publication on crates.io
    - [x] Cargo.toml
    - [ ] Documentation for existing functionality
- [x] Add all tmux subcommands stubs


# Wishlist

- mb function parameter names from tmux source?
- mb better struct fields names
- mb folder structure, separate tmux functions from parse functions
- does `Option<bool>` as function arguments and structure fields make sense
- mb store `PathBuf` or other type for paths in parsed structures?
- optional arguments if they are more than 1 wrap in struct?
- [x] all required (non-optional) arguments move out from struct, use them directly
- callbacks or hooks or smtg like in [Jezza's fork](https://github.com/Jezza/tmux-interface-rs/)
- mb FFI as a C lib?
- [x] enum for pane size specification `[-l size | -p percentage]`
- [ ] mb enum for things like `[size | percentage]` in options?
- mb default most needed struct fields initialized with `new()`?


# Strategy

- additional tmux plugin?
    - [x] no, standalone library (current decision)
        reason: trying to follow UNIX-way, KISS
    - [ ] yes, more options and features possible

- tmux subcommands have optional boolean keys, wrapping method?
    - [x] `Option<bool>` (current decision)
        reason: "mapping" of CLI syntax characters ("12.1 Utility Argument Syntax"
        [IEEE Std 1003.1-2017](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap12.html))
        `tmux lsp [-a] [-t target]`
    - [ ] `bool` directly
        reason: it's simple, for all not boolean keys it's still `Option<T>`

- tmux subcommands have many keys?
    - [ ] pass all keys as function arguments
    - [ ] less than 4 keys - as arguments, more - structure
    - [x] optional keys (> 4) as structure, all required keys as function arguments directly
        (current decision)
        reason: simple function call by default if no args needed
    - [ ] all optional keys (> 1) - as structure, all required keys as arguments?
    - [ ] group optional keys in struct by usage (for example: target as direct
        argument)?

- tmux subcommnads have many optional keys in some structure?
    - [x] `Option<&TmuxSubcommandParameters>` (current decision)
        reason: if all structure fields are `None`, structure itself does not
        to be needed
    - [ ] `&TmuxSubcommandParameters`
