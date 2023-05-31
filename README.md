# tmux_interface

[![Build Status](https://github.com/AntonGepting/tmux-interface-rs/actions/workflows/actions.yml/badge.svg)](https://github.com/AntonGepting/tmux-interface-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/tmux_interface.svg)](https://crates.io/crates/tmux_interface)
[![Documentation](https://docs.rs/tmux_interface/badge.svg)](https://docs.rs/tmux_interface)


## Description

`tmux_interface` is a library for communication with
[TMUX](https://github.com/tmux/tmux) via CLI written in
[Rust](https://www.rust-lang.org/) programming language. The crate
documentation can be found on the
[docs.rs](https://docs.rs/tmux_interface) page.


## Usage


1. Add a dependency in your `Cargo.toml`. Versions below `1.0.0` are
    mostly for development and testing purposes (use them in your projects on
    your own risk, further versions may have different API).

     ```text
     [dependencies]
     tmux_interface = "1.0.0"
     ```

2. Add extern crate in your source file.
     ```
     extern crate tmux_interface;
     ```

3. Use it's functions

     ### Example 1
     ```
     use tmux_interface::{HasSession, KillSession, NewSession, NewWindow, SplitWindow, Tmux};

     let target_session = "example_1";

     // tmux new -d -s example_1 ; neww ; splitw -v
     Tmux::new()
         .add_command(NewSession::new().detached().session_name(target_session))
         .add_command(NewWindow::new())
         .add_command(SplitWindow::new().vertical())
         .output()
         .unwrap();

     // tmux has -t example_1
     let status = Tmux::with_command(HasSession::new().target_session(target_session))
         .status()
         .unwrap()
         .success();

     assert!(status);

     // tmux kill-session -t example_1
     Tmux::with_command(KillSession::new().target_session(target_session))
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

The library was tested using [GitHub Actions](https://github.com/AntonGepting/tmux-interface-rs/actions)
under following conditions:

- OS:
    - [x] Linux (Ubuntu 20.04.2 LTS Focal Fossa, x64)
    - [ ] Windows
    - [ ] MacOS (10.13.6 High Sierra, x64)

- Rust:
    - [x] stable
    - [ ] beta
    - [ ] nightly

- Tmux (covered tmux versions crate features):
    - [ ] master - `tmux_X_X`
    - [x] 3.3a - `tmux_3_3a`
    - [x] 3.3 - `tmux_3_3`
    - [x] 3.2a - `tmux_3_2a`
    - [x] 3.2 - `tmux_3_2`
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
    - [ ] 1.7 - `tmux_1_7`
    - [ ] 1.6 - `tmux_1_6`
    - [ ] 1.5 - `tmux_1_5`
    - [ ] 1.4 - `tmux_1_4` - tmux compilation error
    - [ ] 1.3 - `tmux_1_3` - tmux compilation error
    - [ ] 1.2 - `tmux_1_2` - tmux compilation error
    - [ ] 1.1 - `tmux_1_1` - tmux compilation error
    - [ ] 1.0 - `tmux_1_0` - tmux compilation error
    - [ ] 0.9 - `tmux_0_9` - tmux compilation error
    - [ ] 0.8 - `tmux_0_8` - tmux compilation error


## Contributors

* Kian-Meng Ang ([kianmeng](https://github.com/kianmeng))
* Martin Terneborg ([MTBorg](https://github.com/MTBorg))
* Yanus Poluektovich ([ypoluektovich](https://github.com/ypoluektovich))


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
