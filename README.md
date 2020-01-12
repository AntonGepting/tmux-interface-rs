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

Tmux:
- tmux 3.0a

Rust:
- stable (manually, Travis CI)
- beta (Travis CI)
- nightly (Travis CI)

OS:
- Debian 11 Bullseye, x64 (manually)
- Ubuntu 16.04 Xenial Xerus, x64 (Travis CI)
- MacOS 10.13.6 High Sierra, x64 (Travis CI)

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

    2. TMUX functions (structure similar to [TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html)):

        - [`clients_and_sessions.rs`](src/clients_and_sessions.rs)
        - [`windows_and_panes.rs`](src/windows_and_panes.rs)
        - [`key_bindings.rs`](src/key_bindings.rs)
        - [`options.rs`](src/options.rs)
        - [`hooks.rs`](src/hooks.rs)
        - [`global_and_session_environment.rs`](src/global_and_session_environment.rs)
        - [`status_line.rs`](src/status_line.rs)
        - [`buffers.rs`](src/buffers.rs)
        - [`miscellaneous.rs`](src/miscellaneous.rs)

    3. Unit tests for functions and their parts:

        - [`tmux_interface_tests.rs`](src/tmux_interface_tests.rs)

    4. Parsing functions:

        - [`sessions.rs`](src/sessions.rs) - parse a session list
        - [`session.rs`](src/session.rs) - parse a session
        - [`session_stack.rs`](src/session_stack.rs) - session stack
        - [`windows.rs`](src/windows.rs) - parse a windows list
        - [`window.rs`](src/window.rs) - parse a window
        - [`window_flag.rs`](src/window_flag.rs) - window flag
        - [`panes.rs`](src/panes.rs) - parse a panes list
        - [`pane.rs`](src/pane.rs) - parse a pane
        - [`pane_tabs.rs`](src/pane_tabs.rs) - pane tabs
        - [`tmux_option.rs`](src/tmux_option.rs) - parse an option
        - [`version.rs`](src/version.rs) - parse version response
        - [`layout_cell.rs`](src/layout_cell.rs) - parse layout cell string
        - [`layout_checksum.rs`](src/layout_checksum.rs) - calculate layout checksum
        - [`layout.rs`](src/layout.rs) - parse layot tree string

    5. Unit tests for parsing functions:

        - [`sessions_tests.rs`](src/sessions_tests.rs)
        - [`session_tests.rs`](src/session_tests.rs)
        - [`windows_tests.rs`](src/windows_tests.rs)
        - [`window_tests.rs`](src/window_tests.rs)
        - [`panes_tests.rs`](src/panes_tests.rs)
        - [`pane_tests.rs`](src/pane_tests.rs)
        - [`tmux_option_tests.rs`](src/tmux_option_tests.rs)
        - [`version_tests.rs`](src/version_tests.rs)
        - [`layout_cell_tests.rs`](src/layout_cell_tests.rs)
        - [`layout_checksum_tests.rs`](src/layout_checksum_tests.rs)
        - [`layout_tests.rs`](src/layout_tests.rs)

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
