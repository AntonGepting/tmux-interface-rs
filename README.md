# tmux_interface

[![Build Status](https://travis-ci.com/AntonGepting/tmux-interface-rs.svg?branch=master)](https://travis-ci.com/AntonGepting/tmux-interface-rs)
[![Crates.io](https://img.shields.io/crates/v/tmux_interface.svg)](https://crates.io/crates/tmux_interface)
[![Documentation](https://docs.rs/tmux_interface/badge.svg)](https://docs.rs/tmux_interface)


## Description

`tmux_interface` is a library for communication with
[TMUX](https://github.com/tmux/tmux) via CLI written in
[Rust](https://www.rust-lang.org/) programming language. The crate
documentation can be found on the
[docs.rs](https://docs.rs/tmux_interface) page.


## Usage

1. Add the new crate dependency in your `Cargo.toml`.


    - Using **defaults** (by default: `default = ["tmux_2_8", "cmd_alias"]` will be set)
        ```
        [dependencies]
        tmux_interface = "^0.1.0"
        ```

    - Using **user specified tmux version** (set custom tmux version
      feature, used for conditional compilaton):

        There is an optional dependency parameter `features` in
        `Cargo.toml`, to specify the compatible version of tmux. Because
        different tmux versions may have incompatible CLI changes. Following
        tmux versions are currently supported as library `features`:

        `tmux_0_8`, `tmux_0_9`, `tmux_1_0`, `tmux_1_1`, `tmux_1_2`, `tmux_1_3`,
        `tmux_1_4`, `tmux_1_5`, `tmux_1_6`, `tmux_1_7`, `tmux_1_8`, `tmux_1_9`,
        `tmux_1_9a`, `tmux_2_0`, `tmux_2_1`, `tmux_2_2`, `tmux_2_3`, `tmux_2_4`,
        `tmux_2_5`, `tmux_2_6`, `tmux_2_7`, `tmux_2_8`, `tmux_2_9`, `tmux_2_9a`,
        `tmux_3_0`, `tmux_3_0a`, `tmux_3_1`, `tmux_3_1a`, `tmux_3_1b`,
        `tmux_3_1c`, `tmux_X_X`

        ```
        [dependencies]
        tmux_interface = {
            version = "^0.1.0",
            features = ["tmux_2_6"]
            default-features = false,
        }
        ```

        by default `tmux_2_8` is used. It can be removed with
        `--no-default-features` cargo command line option or with `default-features
        = false` option in `Cargo.toml` in case of using different tmux version.

    - Using library from local repository:
        ```
        [dependencies]
        tmux_interface = {
            version = "^0.1.0",
            path = "../tmux-interface",
            features = ["tmux_X_X"]
        }
        ```

    - Using library from remote repository (for example using developing
      branch):
        ```
        [dependencies]
        tmux_interface = {
            git = "https://github.com/AntonGepting/tmux-interface-rs.git", branch = "dev",
            features = ["tmux_X_X"]
        }
        ```

2. Use library functions in your source file.


    - use methods of the parent structure (`TmuxCommand`):
        ```
        use tmux_interface::TmuxCommand;

        let tmux = TmuxCommand::new();

        tmux.new_session()
            .detached()
            .session_name("example_1")
            .output()
            .unwrap();
        tmux.has_session()
            .target_session("example_1")
            .output()
            .unwrap();
        tmux.kill_session()
            .target_session("example_1")
            .output()
            .unwrap();
        ```

    - use direct methods of the child structures (`NewSession`, `HasSession`,
      `KillSession`, ...):
        ```
        use tmux_interface::{HasSession, KillSession, NewSession};

        NewSession::new()
            .detached()
            .session_name("example_2")
            .output()
            .unwrap();
        HasSession::new()
            .target_session("example_2")
            .output()
            .unwrap();
        KillSession::new()
            .target_session("example_2")
            .output()
            .unwrap();
        ```

## Testing

The library is still in experimental development stage (unstable).
- many features are unimplemented or not well tested
- some APIs/structures/names/... can be changed in the future
- some design patterns of the library can be changed
- almost all library documentation is missing at the moment
- ...

The library was tested on Travis CI (under following conditions: all supported tmux versions features):

- OS:
    - [x] Linux (Ubuntu 16.04 Xenial Xerus, x64)
    - [ ] Windows
    - [ ] MacOS (10.13.6 High Sierra, x64)

- Rust:
    - [x] stable
    - [x] beta
    - [x] nightly

- Tmux:
    - [ ] master - `tmux_X_X`
    - [x] 3.1c - `tmux_3_1c`
    - [x] 3.1b - `tmux_3_1b`
    - [x] 3.1a - `tmux_3_1a`
    - [x] 3.1 - `tmux_3_1`
    - [x] 3.0a - `tmux_3_0a`
    - [x] 3.0 - `tmux_3_0`
    - [x] 2.9a - `tmux_2_9a`
    - [x] 2.9 - `tmux_2_9`
    - [x] 2.8 - `tmux_2_8`
    - [x] 2.7 - `tmux_2_7`
    - [x] 2.6 - `tmux_2_6`
    - [x] 2.5 - `tmux_2_5`
    - [x] 2.4 - `tmux_2_4`
    - [x] 2.3 - `tmux_2_3`
    - [x] 2.2 - `tmux_2_2`
    - [x] 2.1 - `tmux_2_1`
    - [x] 2.0 - `tmux_2_0`
    - [x] 1.9a - `tmux_1_9a`
    - [x] 1.9 - `tmux_1_9`
    - [x] 1.8 - `tmux_1_8`
    - [x] 1.7 - `tmux_1_7`
    - [x] 1.6 - `tmux_1_6`
    - [ ] 1.5 - `tmux_1_5`
    - [ ] 1.4 - `tmux_1_4` - tmux compilation error
    - [ ] 1.3 - `tmux_1_3` - tmux compilation error
    - [ ] 1.2 - `tmux_1_2` - tmux compilation error
    - [ ] 1.1 - `tmux_1_1` - tmux compilation error
    - [ ] 1.0 - `tmux_1_0` - tmux compilation error
    - [ ] 0.9 - `tmux_0_9` - tmux compilation error
    - [ ] 0.8 - `tmux_0_8` - tmux compilation error

## License

`tmux_interface` library is licensed under the MIT license. Please read the
[license file](LICENSE.md) in the repository for more information.


## See also

- [Rust programming language](https://www.rust-lang.org/)
- [crates.io](https://www.crates.io/)
- [docs.rs](https://www.docs.rs/)
- [rust-clippy](https://github.com/rust-lang/rust-clippy)
- [TMUX](https://github.com/tmux/tmux)
- [TMUX man](http://man7.org/linux/man-pages/man1/tmux.1.html)
