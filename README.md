# tmux_interface

[![Build Status](https://travis-ci.com/AntonGepting/tmux-interface-rs.svg?branch=master)](https://travis-ci.com/AntonGepting/tmux-interface-rs)
[![Crates.io](https://img.shields.io/crates/v/tmux_interface.svg)](https://crates.io/crates/tmux_interface)
[![Documentation](https://docs.rs/tmux_interface/badge.svg)](https://docs.rs/tmux_interface)


## Description

`tmux_interface` is a library for communication with
[TMUX](https://github.com/tmux/tmux) via CLI written in [Rust programming
language](https://www.rust-lang.org/). The crate documentation can be found on
the [docs.rs/tmux_interface](https://docs.rs/tmux_interface) page.


## Usage

1. Add the new crate dependency in your `Cargo.toml`.

    - Using **defaults** (`tmux 2.8`)
        ```
        [dependencies]
        tmux_interface = "^0.1.0"
        ```

    - Using **specified tmux version**:

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

    - Using library from remote repository:
        ```
        [dependencies]
        tmux_interface = {
            git = "https://github.com/AntonGepting/tmux-interface-rs.git", branch = "dev",
            features = ["tmux_X_X"]
        }
        ```

2. Use library functions in your source file.

    - Using direct structure initialization:
        ```
        use tmux_interface::{NewSession, TmuxInterface};

        let mut tmux = TmuxInterface::new();
        let new_session = NewSession {
            name: "session_name",
            ..Default::default(),
        };
        tmux.new_session(Some(&new_session)).unwrap();
        ```

    - Using builder pattern:
        ```
        use tmux_interface::{NewSessionBuilder, TmuxInterface};

        let mut tmux = TmuxInterface::new();
        let new_session = NewSessionBuilder::new().session_name("session_name").build();
        tmux.new_session(Some(&new_session)).unwrap();
        ```

    - Using structure modification:
        ```
        use tmux_interface::{NewSession, TmuxInterface};

        let mut tmux = TmuxInterface::new();
        let mut new_session = NewSession {
            ..Default::default(),
        };
        new_session.name = "";
        tmux.new_session(Some(&new_session)).unwrap();
        ```


## Testing

The library is still in experimental development stage (unstable).
- many features are unimplemented or not well tested
- some APIs/structures/names/... can be changed in the future
- some design patterns of the library can be changed
- almost all library documentation is missing at the moment
- ...

The library was tested under following conditions.

Rust:
- stable (manually, Travis CI)
- beta (Travis CI)
- nightly (Travis CI)

OS:
- Debian 11 Bullseye, x64 (manually); tmux 3.0a
- Ubuntu 16.04 Xenial Xerus, x64 (Travis CI); tmux 2.6
- MacOS 10.13.6 High Sierra, x64 (Travis CI); tmux 3.0a


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
