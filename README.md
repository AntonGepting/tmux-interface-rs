# tmux_interface

[![Build Status](https://travis-ci.com/AntonGepting/tmux-interface-rs.svg?branch=master)](https://travis-ci.com/AntonGepting/tmux-interface-rs)
[![Crates.io](https://img.shields.io/crates/v/tmux_interface.svg)](https://crates.io/crates/tmux_interface)
[![Documentation](https://docs.rs/tmux_interface/badge.svg)](https://docs.rs/tmux_interface)

## Description

tmux_interface is a [Rust language](https://www.rust-lang.org/) library for communication with [TMUX](https://github.com/tmux/tmux) via CLI.


## Usage

1. Add a dependency in your `Cargo.toml`

    ```
    [dependencies]
    tmux_interface = "^0.1.0"
    ```

    You can also add `features` to your dependencies entry in `Cargo.toml`, if
    you want to specify the version of tmux you want to use. Different tmux
    versions may have incompatible CLI changes. Following `features` are currently
    supported:

    - `tmux_X_X` - tmux latest, default (based on tmux master branch)
    - `tmux_2_6` - tmux 2.6 (included in Ubuntu 18.04 LTS Bionic Beaver)
    <!--- `tmux_2_1` - tmux 2.1 (included in Ubuntu 16.04 LTS Xenial Xerus) -->
    <!--- `tmux 1_8` - tmux 1.8 (included in Ubuntu 14.04 LTS Trusty Tahr) -->
    <!--- `tmux_1_6` - tmux 1.6 (included in Ubuntu 12.04 LTS Precise Pangolin)-->

    ```
    [dependencies]
    tmux_interface = { version = "^0.1.0", features = ["tmux_2_6"] }
    ```

    by default `tmux_X_X` is used. It can be removed with `--no-default-features`
    cargo command line option or with `default-features = false` option in `Cargo.toml`

    ```
    [dependencies]
    tmux_interface = { version = "^0.1.0", default-features = false, features = ["tmux_2_6"] }
    ```

<!--Add local repository-->
<!--```-->
<!--[dependencies]-->
<!--tmux_interface = { version = "0.0.7", path = "../tmux-interface", features = ["tmux_2_6"] }-->
<!--```-->

<!--```-->
<!--Add remote repository-->
<!--tmux_interface = { git = "https://github.com/AntonGepting/tmux-interface-rs.git", branch = "dev" }-->
<!--```-->


2. Add extern crate and use in your source file

    ```
    extern crate tmux_interface;
    ```

3. Use it's functions
    ```
    use tmux_interface::{AttachSession, NewSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("session_name"),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();
    let attach_session = AttachSession {
        target_session: Some("session_name"),
        ..Default::default()
    };
    tmux.attach_session(Some(&attach)).unwrap();
    tmux.kill_session(None, None, Some("session_name")).unwrap();
    ```


## Misc

- Versions below `0.1.0` are first public releases, mostly for development
and testing purposes. Do not use them in your Projects.

- Used in mosaic - tmux manager

## Testing

The library was tested under following conditions.

Rust:
- stable (manually, Travis CI)
- beta (Travis CI)
- nightly (Travis CI)

OS:
- Debian 11 Bullseye, x64 (manually); tmux 3.0a
- Ubuntu 16.04 Xenial Xerus, x64 (Travis CI); tmux 2.6
- MacOS 10.13.6 High Sierra, x64 (Travis CI); tmux 3.0a

<!--- Structure field names can be chnaged-->

<!--- TmuxInterface::new() required everytime for new commands?-->

<!---
## Project Structure
-->


## Directory Structure

- [`src/`](src/) - crate sources

    1. Common:

        - [`tmux_interface.rs`](src/tmux_interface.rs) - common functions
        - [`error.rs`](src/error.rs) - error propagating functions
        - [`lib.rs`](src/lib.rs) - main library file

    2. TMUX requests sending functions (structure similar to [TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html)):

        - [`src/request/buffers`](src/request/buffers/)

            - [`choose_buffer.rs`](src/request/buffers/choose_buffer.rs)
            - [`clear_history.rs`](src/request/buffers/clear_history.rs)
            - [`delete_buffer.rs`](src/request/buffers/delete_buffer.rs)
            - [`list_buffers.rs`](src/request/buffers/list_buffers.rs)
            - [`load_buffer.rs`](src/request/buffers/load_buffer.rs)
            - [`mod.rs`](src/request/buffers/mod.rs)
            - [`paste_buffer.rs`](src/request/buffers/paste_buffer.rs)
            - [`save_buffer.rs`](src/request/buffers/save_buffer.rs)
            - [`set_buffer.rs`](src/request/buffers/set_buffer.rs)
            - [`show_buffer.rs`](src/request/buffers/show_buffer.rs)

        - [`src/request/clients_and_sessions`](src/request/clients_and_sessions/)

            - [`attach_session.rs`](src/request/clients_and_sessions/attach_session.rs) - (`tmux_X_X`, `tmux_2_6`)
            - [`detach_client.rs`](src/request/clients_and_sessions/detach_client.rs)
            - [`has_session.rs`](src/request/clients_and_sessions/has_session.rs)
            - [`kill_server.rs`](src/request/clients_and_sessions/kill_server.rs)
            - [`kill_session.rs`](src/request/clients_and_sessions/kill_session.rs)
            - [`list_clients.rs`](src/request/clients_and_sessions/list_clients.rs)
            - [`list_commands.rs`](src/request/clients_and_sessions/list_commands.rs)
            - [`list_sessions.rs`](src/request/clients_and_sessions/list_sessions.rs)
            - [`lock_client.rs`](src/request/clients_and_sessions/lock_client.rs)
            - [`lock_session.rs`](src/request/clients_and_sessions/lock_session.rs)
            - [`mod.rs`](src/request/clients_and_sessions/mod.rs)
            - [`new_session.rs`](src/request/clients_and_sessions/new_session.rs)
            - [`refresh_client.rs`](src/request/clients_and_sessions/refresh_client.rs)
            - [`rename_session.rs`](src/request/clients_and_sessions/rename_session.rs)
            - [`show_messages.rs`](src/request/clients_and_sessions/show_messages.rs)
            - [`source_file.rs`](src/request/clients_and_sessions/source_file.rs)
            - [`start_server.rs`](src/request/clients_and_sessions/start_server.rs)
            - [`suspend_client.rs`](src/request/clients_and_sessions/suspend_client.rs)
            - [`switch_client.rs`](src/request/clients_and_sessions/switch_client.rs)

        - [`src/request/global_and_session_environment`](src/request/global_and_session_environment/)

            - [`mod.rs`](src/request/global_and_session_environment/mod.rs)
            - [`set_environment.rs`](src/request/global_and_session_environment/set_environment.rs)
            - [`show_environment.rs`](src/request/global_and_session_environment/show_environment.rs)

        - [`src/request/hooks`](src/request/hooks/)

            - [`mod.rs`](src/request/hooks/mod.rs)
            - [`set_hook.rs`](src/request/hooks/set_hook.rs)
            - [`show_hooks.rs`](src/request/hooks/show_hooks.rs)

        - [`src/request/key_bindings`](src/request/key_bindings/)

            - [`bind_key.rs`](src/request/key_bindings/bind_key.rs)
            - [`list_keys.rs`](src/request/key_bindings/list_keys.rs)
            - [`mod.rs`](src/request/key_bindings/mod.rs)
            - [`send_keys.rs`](src/request/key_bindings/send_keys.rs)
            - [`send_prefix.rs`](src/request/key_bindings/send_prefix.rs)
            - [`unbind_key.rs`](src/request/key_bindings/unbind_key.rs)

        - [`src/request/miscellaneous`](src/request/miscellaneous/)

            - [`clock_mode.rs`](src/request/miscellaneous/clock_mode.rs)
            - [`if_shell.rs`](src/request/miscellaneous/if_shell.rs)
            - [`lock_server.rs`](src/request/miscellaneous/lock_server.rs)
            - [`mod.rs`](src/request/miscellaneous/mod.rs)
            - [`run_shell.rs`](src/request/miscellaneous/run_shell.rs)
            - [`wait_for.rs`](src/request/miscellaneous/wait_for.rs)

        - [`src/request/options`](src/request/options/)

            - [`mod.rs`](src/request/options/mod.rs)
            - [`set_option.rs`](src/request/options/set_option.rs)
            - [`set_window_option.rs`](src/request/options/set_window_option.rs)
            - [`show_options.rs`](src/request/options/show_options.rs)
            - [`show_window_options.rs`](src/request/options/show_window_options.rs)

        - [`src/request/status_line`](src/request/status_line/)

            - [`command_prompt.rs`](src/request/status_line/command_prompt.rs)
            - [`confirm_before.rs`](src/request/status_line/confirm_before.rs)
            - [`display_menu.rs`](src/request/status_line/display_menu.rs)
            - [`display_message.rs`](src/request/status_line/display_message.rs)
            - [`mod.rs`](src/request/status_line/mod.rs)

        - [`src/request/windows_and_panes`](src/request/windows_and_panes/)

            - [`break_pane.rs`](src/request/windows_and_panes/break_pane.rs)
            - [`break_pane_tests.rs`](src/request/windows_and_panes/break_pane_tests.rs)
            - [`capture_pane.rs`](src/request/windows_and_panes/capture_pane.rs)
            - [`capture_pane_tests.rs`](src/request/windows_and_panes/capture_pane_tests.rs)
            - [`choose_client.rs`](src/request/windows_and_panes/choose_client.rs)
            - [`choose_client_tests.rs`](src/request/windows_and_panes/choose_client_tests.rs)
            - [`choose_tree.rs`](src/request/windows_and_panes/choose_tree.rs)
            - [`choose_tree_tests.rs`](src/request/windows_and_panes/choose_tree_tests.rs)
            - [`copy_mode.rs`](src/request/windows_and_panes/copy_mode.rs)
            - [`copy_mode_tests.rs`](src/request/windows_and_panes/copy_mode_tests.rs)
            - [`display_pane.rs`](src/request/windows_and_panes/display_pane.rs)
            - [`display_pane_tests.rs`](src/request/windows_and_panes/display_pane_tests.rs)
            - [`find_window.rs`](src/request/windows_and_panes/find_window.rs)
            - [`find_window_tests.rs`](src/request/windows_and_panes/find_window_tests.rs)
            - [`join_pane.rs`](src/request/windows_and_panes/join_pane.rs)
            - [`kill_pane.rs`](src/request/windows_and_panes/kill_pane.rs)
            - [`kill_window.rs`](src/request/windows_and_panes/kill_window.rs)
            - [`last_pane.rs`](src/request/windows_and_panes/last_pane.rs)
            - [`last_window.rs`](src/request/windows_and_panes/last_window.rs)
            - [`link_window.rs`](src/request/windows_and_panes/link_window.rs)
            - [`list_panes.rs`](src/request/windows_and_panes/list_panes.rs)
            - [`list_windows.rs`](src/request/windows_and_panes/list_windows.rs)
            - [`mod.rs`](src/request/windows_and_panes/mod.rs)
            - [`move_pane.rs`](src/request/windows_and_panes/move_pane.rs)
            - [`move_window.rs`](src/request/windows_and_panes/move_window.rs)
            - [`new_window.rs`](src/request/windows_and_panes/new_window.rs)
            - [`next_layout.rs`](src/request/windows_and_panes/next_layout.rs)
            - [`next_window.rs`](src/request/windows_and_panes/next_window.rs)
            - [`pipe_pane.rs`](src/request/windows_and_panes/pipe_pane.rs)
            - [`previous_layout.rs`](src/request/windows_and_panes/previous_layout.rs)
            - [`previous_window.rs`](src/request/windows_and_panes/previous_window.rs)
            - [`rename_window.rs`](src/request/windows_and_panes/rename_window.rs)
            - [`resize_pane.rs`](src/request/windows_and_panes/resize_pane.rs)
            - [`resize_window.rs`](src/request/windows_and_panes/resize_window.rs)
            - [`respawn_pane.rs`](src/request/windows_and_panes/respawn_pane.rs)
            - [`respawn_window.rs`](src/request/windows_and_panes/respawn_window.rs)
            - [`rotate_window.rs`](src/request/windows_and_panes/rotate_window.rs)
            - [`select_layout.rs`](src/request/windows_and_panes/select_layout.rs)
            - [`select_pane.rs`](src/request/windows_and_panes/select_pane.rs)
            - [`select_window.rs`](src/request/windows_and_panes/select_window.rs)
            - [`split_window.rs`](src/request/windows_and_panes/split_window.rs)
            - [`swap_pane.rs`](src/request/windows_and_panes/swap_pane.rs)
            - [`swap_window.rs`](src/request/windows_and_panes/swap_window.rs)
            - [`unlink_window.rs`](src/request/windows_and_panes/unlink_window.rs)

        - [`mod.rs`](src/request/mod.rs)

    3. TMUX response parsing functions

        - [`src/response/session`](src/response/session)

            - [`sessions.rs`](src/sessions.rs) - parse a session list
            - [`session.rs`](src/session.rs) - parse a session
            - [`session_stack.rs`](src/session_stack.rs) - session stack

            - [`sessions_tests.rs`](src/sessions_tests.rs)
            - [`session_tests.rs`](src/session_tests.rs)

        - [`src/response/window`](src/response/window)

            - [`windows.rs`](src/windows.rs) - parse a windows list
            - [`window.rs`](src/window.rs) - parse a window
            - [`window_flag.rs`](src/window_flag.rs) - window flag

            - [`windows_tests.rs`](src/windows_tests.rs)
            - [`window_tests.rs`](src/window_tests.rs)

        - [`src/response/pane`](src/response/pane)

            - [`panes.rs`](src/panes.rs) - parse a panes list
            - [`pane.rs`](src/pane.rs) - parse a pane
            - [`pane_tabs.rs`](src/pane_tabs.rs) - pane tabs

            - [`panes_tests.rs`](src/panes_tests.rs)
            - [`pane_tests.rs`](src/pane_tests.rs)

        - [`src/response/layout`](src/response/layout)

            - [`layout_cell.rs`](src/layout_cell.rs) - parse layout cell string
            - [`layout_checksum.rs`](src/layout_checksum.rs) - calculate layout checksum
            - [`layout.rs`](src/layout.rs) - parse layot tree string

            - [`layout_cell_tests.rs`](src/layout_cell_tests.rs)
            - [`layout_checksum_tests.rs`](src/layout_checksum_tests.rs)
            - [`layout_tests.rs`](src/layout_tests.rs)

    3. Unit tests for functions and their parts:

        - [`tmux_interface_tests.rs`](src/tmux_interface_tests.rs)
        - [`tmux_option.rs`](src/tmux_option.rs) - parse an option
        - [`version.rs`](src/version.rs) - parse version response

    5. Unit tests for parsing functions:

        - [`tmux_option_tests.rs`](src/tmux_option_tests.rs)
        - [`version_tests.rs`](src/version_tests.rs)

- [`tests/`](tests/) - crate integration tests (multiple functions):

    - [`issue1.rs`](tests/issue1.rs) - issue #1 tests
    - [`sessions_tests.rs`](tests/sessions_tests.rs) - sessions tests
    - [`windows_tests.rs`](tests/windows_tests.rs) - windows tests
    - [`panes_tests.rs`](tests/panes_tests.rs) - panes tests
    - [`tmux_error_mock.sh`](tests/tmux_error_mock.sh) - bash script for testing of tmux error handling functions
    - [`tmux_interface.rs`](tests/tmux_interface.rs)
    - [`tmux_mock.sh`](tests/tmux_mock.sh) - bash script can be used instead of tmux binary, for simple logging
        (sniffing) intercommmunication between library functions and tmux
    - [`tmux_test.sh`](tests/tmux_test.sh) - bash script for output testing of tmux functions

- [`Cargo.toml`](Cargo.toml) - crate configuration ([File Format](https://doc.rust-lang.org/cargo/reference/manifest.html))
- [`CHANGELOG.md`](CHANGELOG.md) - version history
- [`clippy.toml`](clippy.toml) - Clippy configuration file ([Clippy](https://github.com/rust-lang/rust-clippy#configuration))
- [`.editorconfig`](.editorconfig) - consistent coding style configuration ([File Format](https://editorconfig.org/#file-format-details))
- [`LICENSE.md`](LICENSE.md) - license text
- [`README.md`](README.md) - common information (this file)
- [`ROADMAP.md`](ROADMAP.md) - future goals, wishlist, ideas
- [`rustfmt.toml`](rustfmt.toml) - rustfmt configuration file ([rustfmt](https://github.com/rust-lang/rustfmt#configuring-rustfmt))
- [`rust-toolchain`](rust-toolchain) - rustup toolchain configuration file ([rustup](https://github.com/rust-lang/rustup.rs#the-toolchain-file))
- [`.travis.yml`](.travis.yml) - travis CI configuration ([File Format](https://docs.travis-ci.com/user/tutorial/))


## Contributing

If you are interested in this project and you have:

- a bug report
- any feature suggestion
- nice code contribution
- an improvment idea
- ...

You are allways welcome, please feel free to use following links to contact me
and/or to contribute to the project:

- [Write E-Mail](mailto:anton.gepting@gmail.com)
- [Open issue](https://github.com/AntonGepting/tmux-interface-rs/issues/new)
- [Send pull request](https://github.com/AntonGepting/tmux-interface-rs/pulls)


## License

tmux_interface is licensed under the MIT license. Please read the license
file in this repository for more information.
