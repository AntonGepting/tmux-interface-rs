# tmux_interface

Tmux Interface is a rust language library for communication with TMUX via CLI.


## Usage

1. Add a dependency in your `Cargo.toml`

    ```
    [dependencies]
    tmux_interface = "*"
    ```

2. Add extern crate and use in your source file

    ```
    extern crate tmux_interface;
    ```

3. Use it's functions
    ```
    let tmux = TmuxInterface::new(None);
    tmux.list_sessions();
    ```


## Misc

Used in mosaic - tmux manager

Tested on: tmux 2.8


## License

tmux interface is licensed under the MIT license. Please read the license file in this repository for more information.
