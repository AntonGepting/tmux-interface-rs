# tmux_interface

[![Build Status](https://travis-ci.com/AntonGepting/tmux-interface-rs.svg?branch=master)](https://travis-ci.com/AntonGepting/tmux-interface-rs)
[![Crates.io](https://img.shields.io/crates/v/tmux_interface.svg)](https://crates.io/crates/tmux_interface)
[![Documentation](https://docs.rs/tmux_interface/badge.svg)](https://docs.rs/tmux_interface)

## Description

tmux_interface is a rust language library for communication with TMUX via CLI.


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
    let tmux = TmuxInterface::new();
    tmux.list_sessions();
    ```


## Misc

- Versions below `0.1.0` are first public releases, mostly for development
and testing purposes. Do not use them in your Projects.

- Used in mosaic - tmux manager

- Tested on: tmux 2.8

<!---
## Project Structure
-->

## Library Functions

Principles:

1. Function names and their grouping are inherited from tmux manual
2. Function arguments and their optionality inherited from tmux manual
3. Functions can have max. 4 arguments, otherwise a structure will be used


## Folder Structure

- [`src/`](src/) - crate sources

    1. Common:

        - [`tmux_interface.rs`](src/tmux_interface.rs) - common functions
        - [`tmux_interface_error.rs`](src/tmux_interface_error.rs) - error propagating functions
        - [`lib.rs`](src/lib.rs) - main library file

    2. TMUX functions (structure like in TMUX manual):

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
        - [`windows.rs`](src/windows.rs) - parse a windows list
        - [`window.rs`](src/window.rs) - parse a window
        - [`panes.rs`](src/panes.rs) - parse a panes list
        - [`pane.rs`](src/pane.rs) - parse a pane
        - [`tmux_option.rs`](src/tmux_option.rs) - parse an option

    5. Unit tests for parsing functions:

        - [`sessions_tests.rs`](src/sessions_tests.rs)
        - [`session_tests.rs`](src/session_tests.rs)
        - [`windows_tests.rs`](src/windows_tests.rs)
        - [`window_tests.rs`](src/window_tests.rs)
        - [`panes_tests.rs`](src/panes_tests.rs)
        - [`pane_tests.rs`](src/pane_tests.rs)
        - [`tmux_option_tests.rs`](src/tmux_option_tests.rs)

- [`tests/`](tests/) - crate integration tests (multiple functions):

    - [`tmux_interface.rs`](tests/tmux_interface.rs)
    - [`tmux_mock.sh`](tests/tmux_mock.sh) - bash script can be used instead of tmux binary, for simple logging
        (sniffing) intercommmunication between library functions and tmux
    - [`tmux_test.sh`](tests/tmux_test.sh) - bash script for output testing of tmux functions

- `README.md` - common information (this file)
- [`LICENSE.md`](LICENSE.md) - license text
- [`ROADMAP.md`](ROADMAP.md) - future goals, wishlist, ideas
- [`CHANGELOG.md`](CHANGELOG.md) - version history
- [`Cargo.toml`](Cargo.toml) - crate configuration ([File Format](https://doc.rust-lang.org/cargo/reference/manifest.html))
- [`.travis.yml`](.travis.yml) - travis CI configuration ([File Format](https://docs.travis-ci.com/user/tutorial/))
- [`.editorconfig`](.editorconfig) - consisten conding style configuration ([File Format](https://editorconfig.org/#file-format-details))


## License

tmux_interface is licensed under the MIT license. Please read the license
file in this repository for more information.
