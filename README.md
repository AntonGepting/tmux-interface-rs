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

1. Add a dependency in your `Cargo.toml`.
    ```
    [dependencies]
    tmux_interface = "^0.0.1"
    ```

    You can also add `features` to your dependencies entry in `Cargo.toml`, if
    you want to specify the version of `tmux` you want to use (because
    different tmux versions may have incompatible CLI changes). Following
    `features` are currently supported:

    - `tmux_X_X` - tmux latest, default (based on tmux master branch)
    - `tmux_2_6` - tmux 2.6 (included by default in Ubuntu 18.04 LTS Bionic Beaver)
    <!--- `tmux_2_1` - tmux 2.1 (included by default in Ubuntu 16.04 LTS Xenial Xerus) -->
    <!--- `tmux 1_8` - tmux 1.8 (included by default in Ubuntu 14.04 LTS Trusty Tahr) -->
    <!--- `tmux_1_6` - tmux 1.6 (included by default in Ubuntu 12.04 LTS Precise Pangolin)-->

    ```
    [dependencies]
    tmux_interface = { version = "^0.0.1", features = ["tmux_2_6"] }
    ```

    by default `tmux_X_X` is used. It can be removed with
    `--no-default-features` cargo command line option or with `default-features
    = false` option in `Cargo.toml`

    ```
    [dependencies]
    tmux_interface = { version = "^0.0.1", default-features = false, features = ["tmux_2_6"] }
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

2. Use library functions in your source file.

    ```
    use tmux_interface::{NewSessionBuilder, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    let new_session = NewSessionBuilder::new().session_name("session_name").build();
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

<!--- [Rust programming language](https://www.rust-lang.org/)-->
<!--- [crates.io](https://www.crates.io/)-->
<!--- [docs.rs](https://www.docs.rs/)-->
<!--- [rust-clippy](https://github.com/rust-lang/rust-clippy)-->
<!--- [TMUX](https://github.com/tmux/tmux)-->
<!--- [TMUX man](http://man7.org/linux/man-pages/man1/tmux.1.html)-->
