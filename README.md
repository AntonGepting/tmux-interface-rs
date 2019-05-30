# tmux_interface

Ubuntu Trusty 14.04: [![Build Status](https://travis-ci.com/AntonGepting/tmux-interface-rs.svg?branch=master)](https://travis-ci.com/AntonGepting/tmux-interface-rs)

Tmux Interface is a rust language library for communication with TMUX via CLI.


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


## License

tmux interface is licensed under the MIT license. Please read the license
file in this repository for more information.
