# Roadmap

Some goals for further versions and current development are listed below.
Please [contact me](mailto:anton.gepting@gmail.com) if any feature is missing or if you have a request,
an improvment, an idea etc.


**tmux interface v1.0.0**

- [ ] Support all tmux subcommands
    - [x] Clients and Sessions
    - [ ] Windows and Panes
    - [ ] Key Bindings
    - [ ] Options
    - [ ] Hooks
    - [ ] Global and Session Environments
    - [ ] Status Line
    - [ ] Buffers
    - [ ] Miscellaneous
- [ ] Documentation
    - [ ] Clients and Sessions
    - [ ] Windows and Panes
    - [ ] Key Bindings
    - [ ] Options
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
    - [ ] Hooks
    - [ ] Global and Session Environments
    - [ ] Status Line
    - [ ] Buffers
    - [ ] Miscellaneous
- [ ] Freeze basic architecture
- [ ] Freeze API


**tmux interface v0.1.0**

- [ ] Support basic tmux subcommands (references:
[tmuxinator](https://github.com/tmuxinator/tmuxinator),
[libtmux](https://github.com/tmux-python/libtmux), rust crates using tmux)
- [ ] Parse full information from tmux responses into structures
- [ ] Prepare documentation
- [ ] Prepare tests
- [ ] Function results and errors


**tmux interface v0.0.2**

- [ ] Documentation for all existing functionality and items


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
- mb better error reporting (mb add enum)
- mb function parameter names from tmux source?
- mb tmux plugin for more options?
- mb folder structure, separate tmux functions from parse functions
