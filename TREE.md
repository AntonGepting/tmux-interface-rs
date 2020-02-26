<!--## Table of contents-->

<!--* [Description](README.md#description)-->
<!--* [Usage](USAGE.md#usage)-->
<!--* [Testing](TESTING.md#testing)-->
<!--* [Directory structure](TREE.md#directory-structure)-->
<!--* [Contributing](CONTIBUTING.md#contributing)-->
<!--* [License](README.md#license)-->


## Directory structure

- [`src/`](src/) - crate sources

    1. Common:

        - [`tmux_interface.rs`](src/tmux_interface.rs) - common functions
        - [`error.rs`](src/error.rs) - error propagating functions
        - [`lib.rs`](src/lib.rs) - main library file


        - [`src/common`](src/common/) - set/get options functions
            - [`mod.rs`](src/common/mod.rs) - module file

            - [`server_options.rs`](src/common/server_options.rs) - server options
            - [`session_options.rs`](src/common/session_options.rs) - session options
            - [`window_options.rs`](src/common/window_options.rs) - window options
            - [`pane_options.rs`](src/common/pane_options.rs) - pane options

            - [`pane_options_tests.rs`](src/common/pane_options_tests.rs) - pane options tests
            - [`server_options_tests.rs`](src/common/pane_options_tests.rs) - server options tests
            - [`session_options_tests.rs`](src/common/session_options_tests.rs) - session options tests
            - [`window_options_tests.rs`](src/common/window_options_tests.rs) - window options tests


    2. TMUX requests sending functions (structure similar to [TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html)):

        - [`src/request/buffers`](src/request/buffers/) ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#BUFFERS))
            - [`mod.rs`](src/request/buffers/mod.rs)

            - [`choose_buffer.rs`](src/request/buffers/choose_buffer.rs)
            - [`clear_history.rs`](src/request/buffers/clear_history.rs)
            - [`delete_buffer.rs`](src/request/buffers/delete_buffer.rs)
            - [`list_buffers.rs`](src/request/buffers/list_buffers.rs)
            - [`load_buffer.rs`](src/request/buffers/load_buffer.rs)
            - [`paste_buffer.rs`](src/request/buffers/paste_buffer.rs)
            - [`save_buffer.rs`](src/request/buffers/save_buffer.rs)
            - [`set_buffer.rs`](src/request/buffers/set_buffer.rs)
            - [`show_buffer.rs`](src/request/buffers/show_buffer.rs)

            - [`choose_buffer_tests.rs`](src/request/buffers/choose_buffer_tests.rs)
            - [`clear_history_tests.rs`](src/request/buffers/clear_history_tests.rs)
            - [`delete_buffer_tests.rs`](src/request/buffers/delete_buffer_tests.rs)
            - [`list_buffers_tests.rs`](src/request/buffers/list_buffers_tests.rs)
            - [`load_buffer_tests.rs`](src/request/buffers/load_buffer_tests.rs)
            - [`paste_buffer_tests.rs`](src/request/buffers/paste_buffer_tests.rs)
            - [`save_buffer_tests.rs`](src/request/buffers/save_buffer_tests.rs)
            - [`set_buffer_tests.rs`](src/request/buffers/set_buffer_tests.rs)
            - [`show_buffer_tests.rs`](src/request/buffers/show_buffer_tests.rs)

        - [`src/request/clients_and_sessions`](src/request/clients_and_sessions/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS))
            - [`mod.rs`](src/request/clients_and_sessions/mod.rs)

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
            - [`new_session.rs`](src/request/clients_and_sessions/new_session.rs)
            - [`refresh_client.rs`](src/request/clients_and_sessions/refresh_client.rs)
            - [`rename_session.rs`](src/request/clients_and_sessions/rename_session.rs)
            - [`show_messages.rs`](src/request/clients_and_sessions/show_messages.rs)
            - [`source_file.rs`](src/request/clients_and_sessions/source_file.rs)
            - [`start_server.rs`](src/request/clients_and_sessions/start_server.rs)
            - [`suspend_client.rs`](src/request/clients_and_sessions/suspend_client.rs)
            - [`switch_client.rs`](src/request/clients_and_sessions/switch_client.rs)

            - [`attach_session_tests.rs`](src/request/clients_and_sessions/attach_session_tests.rs) - (`tmux_X_X`, `tmux_2_6`)
            - [`detach_client_tests.rs`](src/request/clients_and_sessions/detach_client_tests.rs)
            - [`has_session_tests.rs`](src/request/clients_and_sessions/has_session_tests.rs)
            - [`kill_server_tests.rs`](src/request/clients_and_sessions/kill_server_tests.rs)
            - [`kill_session_tests.rs`](src/request/clients_and_sessions/kill_session_tests.rs)
            - [`list_clients_tests.rs`](src/request/clients_and_sessions/list_clients_tests.rs)
            - [`list_commands_tests.rs`](src/request/clients_and_sessions/list_commands_tests.rs)
            - [`list_sessions_tests.rs`](src/request/clients_and_sessions/list_sessions_tests.rs)
            - [`lock_client_tests.rs`](src/request/clients_and_sessions/lock_client_tests.rs)
            - [`lock_session_tests.rs`](src/request/clients_and_sessions/lock_session_tests.rs)
            - [`new_session_tests.rs`](src/request/clients_and_sessions/new_session_tests.rs)
            - [`refresh_client_tests.rs`](src/request/clients_and_sessions/refresh_client_tests.rs)
            - [`rename_session_tests.rs`](src/request/clients_and_sessions/rename_session_tests.rs)
            - [`show_messages_tests.rs`](src/request/clients_and_sessions/show_messages_tests.rs)
            - [`source_file_tests.rs`](src/request/clients_and_sessions/source_file_tests.rs)
            - [`start_server_tests.rs`](src/request/clients_and_sessions/start_server_tests.rs)
            - [`suspend_client_tests.rs`](src/request/clients_and_sessions/suspend_client_tests.rs)
            - [`switch_client_tests.rs`](src/request/clients_and_sessions/switch_client_tests.rs)

        - [`src/request/global_and_session_environment`](src/request/global_and_session_environment/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#GLOBAL_AND_SESSION_ENVIRONMENT))

            - [`mod.rs`](src/request/global_and_session_environment/mod.rs)

            - [`set_environment.rs`](src/request/global_and_session_environment/set_environment.rs)
            - [`show_environment.rs`](src/request/global_and_session_environment/show_environment.rs)

            - [`set_environment_tests.rs`](src/request/global_and_session_environment/set_environment_tests.rs)
            - [`show_environment_tests.rs`](src/request/global_and_session_environment/show_environment_tests.rs)

        - [`src/request/hooks`](src/request/hooks/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#HOOKS))

            - [`mod.rs`](src/request/hooks/mod.rs)

            - [`set_hook.rs`](src/request/hooks/set_hook.rs)
            - [`show_hooks.rs`](src/request/hooks/show_hooks.rs)

            - [`set_hook_tests.rs`](src/request/hooks/set_hook_tests.rs)
            - [`show_hooks_tests.rs`](src/request/hooks/show_hooks_tests.rs)

        - [`src/request/key_bindings`](src/request/key_bindings/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#KEY_BINDINGS))

            - [`mod.rs`](src/request/key_bindings/mod.rs)

            - [`bind_key.rs`](src/request/key_bindings/bind_key.rs)
            - [`list_keys.rs`](src/request/key_bindings/list_keys.rs)
            - [`send_keys.rs`](src/request/key_bindings/send_keys.rs)
            - [`send_prefix.rs`](src/request/key_bindings/send_prefix.rs)
            - [`unbind_key.rs`](src/request/key_bindings/unbind_key.rs)

            - [`bind_key_tests.rs`](src/request/key_bindings/bind_key_tests.rs)
            - [`list_keys_tests.rs`](src/request/key_bindings/list_keys_tests.rs)
            - [`send_keys_tests.rs`](src/request/key_bindings/send_keys_tests.rs)
            - [`send_prefix_tests.rs`](src/request/key_bindings/send_prefix_tests.rs)
            - [`unbind_key_tests.rs`](src/request/key_bindings/unbind_key_tests.rs)

        - [`src/request/miscellaneous`](src/request/miscellaneous/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#MISCELLANEOUS))

            - [`mod.rs`](src/request/miscellaneous/mod.rs)

            - [`clock_mode.rs`](src/request/miscellaneous/clock_mode.rs)
            - [`if_shell.rs`](src/request/miscellaneous/if_shell.rs)
            - [`lock_server.rs`](src/request/miscellaneous/lock_server.rs)
            - [`run_shell.rs`](src/request/miscellaneous/run_shell.rs)
            - [`wait_for.rs`](src/request/miscellaneous/wait_for.rs)

            - [`clock_mode_tests.rs`](src/request/miscellaneous/clock_mode_tests.rs)
            - [`if_shell_tests.rs`](src/request/miscellaneous/if_shell_tests.rs)
            - [`lock_server_tests.rs`](src/request/miscellaneous/lock_server_tests.rs)
            - [`run_shell_tests.rs`](src/request/miscellaneous/run_shell_tests.rs)
            - [`wait_for_tests.rs`](src/request/miscellaneous/wait_for_tests.rs)

        - [`src/request/options`](src/request/options/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#OPTIONS))

            - [`mod.rs`](src/request/options/mod.rs)

            - [`set_option.rs`](src/request/options/set_option.rs)
            - [`set_window_option.rs`](src/request/options/set_window_option.rs)
            - [`show_options.rs`](src/request/options/show_options.rs)
            - [`show_window_options.rs`](src/request/options/show_window_options.rs)

            - [`set_option_tests.rs`](src/request/options/set_option_tests.rs)
            - [`set_window_option_tests.rs`](src/request/options/set_window_option_tests.rs)
            - [`show_options_tests.rs`](src/request/options/show_options_tests.rs)
            - [`show_window_options_tests.rs`](src/request/options/show_window_options_tests.rs)

        - [`src/request/status_line`](src/request/status_line/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#STATUS_LINE))

            - [`mod.rs`](src/request/status_line/mod.rs)

            - [`command_prompt.rs`](src/request/status_line/command_prompt.rs)
            - [`confirm_before.rs`](src/request/status_line/confirm_before.rs)
            - [`display_menu.rs`](src/request/status_line/display_menu.rs)
            - [`display_message.rs`](src/request/status_line/display_message.rs)

            - [`command_prompt_tests.rs`](src/request/status_line/command_prompt_tests.rs)
            - [`confirm_before_tests.rs`](src/request/status_line/confirm_before_tests.rs)
            - [`display_menu_tests.rs`](src/request/status_line/display_menu_tests.rs)
            - [`display_message_tests.rs`](src/request/status_line/display_message_tests.rs)

        - [`src/request/windows_and_panes`](src/request/windows_and_panes/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES))

            - [`mod.rs`](src/request/windows_and_panes/mod.rs)

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

            - [`break_pane_tests.rs`](src/request/windows_and_panes/break_pane_tests.rs)
            - [`break_pane_tests_tests.rs`](src/request/windows_and_panes/break_pane_tests_tests.rs)
            - [`capture_pane_tests.rs`](src/request/windows_and_panes/capture_pane_tests.rs)
            - [`capture_pane_tests_tests.rs`](src/request/windows_and_panes/capture_pane_tests_tests.rs)
            - [`choose_client_tests.rs`](src/request/windows_and_panes/choose_client_tests.rs)
            - [`choose_client_tests_tests.rs`](src/request/windows_and_panes/choose_client_tests_tests.rs)
            - [`choose_tree_tests.rs`](src/request/windows_and_panes/choose_tree_tests.rs)
            - [`choose_tree_tests_tests.rs`](src/request/windows_and_panes/choose_tree_tests_tests.rs)
            - [`copy_mode_tests.rs`](src/request/windows_and_panes/copy_mode_tests.rs)
            - [`copy_mode_tests_tests.rs`](src/request/windows_and_panes/copy_mode_tests_tests.rs)
            - [`display_pane_tests.rs`](src/request/windows_and_panes/display_pane_tests.rs)
            - [`display_pane_tests_tests.rs`](src/request/windows_and_panes/display_pane_tests_tests.rs)
            - [`find_window_tests.rs`](src/request/windows_and_panes/find_window_tests.rs)
            - [`find_window_tests_tests.rs`](src/request/windows_and_panes/find_window_tests_tests.rs)
            - [`join_pane_tests.rs`](src/request/windows_and_panes/join_pane_tests.rs)
            - [`kill_pane_tests.rs`](src/request/windows_and_panes/kill_pane_tests.rs)
            - [`kill_window_tests.rs`](src/request/windows_and_panes/kill_window_tests.rs)
            - [`last_pane_tests.rs`](src/request/windows_and_panes/last_pane_tests.rs)
            - [`last_window_tests.rs`](src/request/windows_and_panes/last_window_tests.rs)
            - [`link_window_tests.rs`](src/request/windows_and_panes/link_window_tests.rs)
            - [`list_panes_tests.rs`](src/request/windows_and_panes/list_panes_tests.rs)
            - [`list_windows_tests.rs`](src/request/windows_and_panes/list_windows_tests.rs)
            - [`move_pane_tests.rs`](src/request/windows_and_panes/move_pane_tests.rs)
            - [`move_window_tests.rs`](src/request/windows_and_panes/move_window_tests.rs)
            - [`new_window_tests.rs`](src/request/windows_and_panes/new_window_tests.rs)
            - [`next_layout_tests.rs`](src/request/windows_and_panes/next_layout_tests.rs)
            - [`next_window_tests.rs`](src/request/windows_and_panes/next_window_tests.rs)
            - [`pipe_pane_tests.rs`](src/request/windows_and_panes/pipe_pane_tests.rs)
            - [`previous_layout_tests.rs`](src/request/windows_and_panes/previous_layout_tests.rs)
            - [`previous_window_tests.rs`](src/request/windows_and_panes/previous_window_tests.rs)
            - [`rename_window_tests.rs`](src/request/windows_and_panes/rename_window_tests.rs)
            - [`resize_pane_tests.rs`](src/request/windows_and_panes/resize_pane_tests.rs)
            - [`resize_window_tests.rs`](src/request/windows_and_panes/resize_window_tests.rs)
            - [`respawn_pane_tests.rs`](src/request/windows_and_panes/respawn_pane_tests.rs)
            - [`respawn_window_tests.rs`](src/request/windows_and_panes/respawn_window_tests.rs)
            - [`rotate_window_tests.rs`](src/request/windows_and_panes/rotate_window_tests.rs)
            - [`select_layout_tests.rs`](src/request/windows_and_panes/select_layout_tests.rs)
            - [`select_pane_tests.rs`](src/request/windows_and_panes/select_pane_tests.rs)
            - [`select_window_tests.rs`](src/request/windows_and_panes/select_window_tests.rs)
            - [`split_window_tests.rs`](src/request/windows_and_panes/split_window_tests.rs)
            - [`swap_pane_tests.rs`](src/request/windows_and_panes/swap_pane_tests.rs)
            - [`swap_window_tests.rs`](src/request/windows_and_panes/swap_window_tests.rs)
            - [`unlink_window_tests.rs`](src/request/windows_and_panes/unlink_window_tests.rs)

        - [`mod.rs`](src/request/mod.rs)

        - [`target_session.rs`](src/request/target_session.rs) - TargetSession object
        - [`target_window.rs`](src/request/target_window.rs) - TargetWindow & TargetWindowEx objects
        - [`target_pane.rs`](src/request/target_pane.rs) - TargetPane & TargetPaneEx objects

        - [`target_session_tests.rs`](src/request/target_session_tests.rs)
        - [`target_window_tests.rs`](src/request/target_window_tests.rs)
        - [`target_pane_tests.rs`](src/request/target_pane_tests.rs)


    3. TMUX response parsing functions

        - [`src/response`](src/response)
            - [`mod.rs`](src/response/mod.rs)

        - [`src/response/layout`](src/response/layout)
            - [`mod.rs`](src/response/layout/mod.rs)

            - [`layout_cell.rs`](src/layout_cell.rs) - parse layout cell string
            - [`layout_checksum.rs`](src/layout_checksum.rs) - calculate layout checksum
            - [`layout.rs`](src/layout.rs) - parse layot tree string

            - [`layout_cell_tests.rs`](src/layout_cell_tests.rs)
            - [`layout_checksum_tests.rs`](src/layout_checksum_tests.rs)
            - [`layout_tests.rs`](src/layout_tests.rs)

        - [`src/response/session`](src/response/session)
            - [`mod.rs`](src/response/session/mod.rs)

            - [`sessions.rs`](src/sessions.rs) - parse a session list
            - [`session.rs`](src/session.rs) - parse a session
            - [`session_stack.rs`](src/session_stack.rs) - session stack

            - [`sessions_tests.rs`](src/sessions_tests.rs)
            - [`session_tests.rs`](src/session_tests.rs)

        - [`src/response/window`](src/response/window)
            - [`mod.rs`](src/response/window/mod.rs)

            - [`windows.rs`](src/windows.rs) - parse a windows list
            - [`window.rs`](src/window.rs) - parse a window
            - [`window_flag.rs`](src/window_flag.rs) - window flag

            - [`windows_tests.rs`](src/windows_tests.rs)
            - [`window_tests.rs`](src/window_tests.rs)

        - [`src/response/pane`](src/response/pane)
            - [`mod.rs`](src/response/pane/mod.rs)

            - [`panes.rs`](src/panes.rs) - parse a panes list
            - [`pane.rs`](src/pane.rs) - parse a pane
            - [`pane_tabs.rs`](src/pane_tabs.rs) - pane tabs

            - [`panes_tests.rs`](src/panes_tests.rs)
            - [`pane_tests.rs`](src/pane_tests.rs)

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

- [`.editorconfig`](.editorconfig) - consistent coding style configuration ([File Format](https://editorconfig.org/#file-format-details))
- [`Cargo.toml`](Cargo.toml) - crate configuration ([File Format](https://doc.rust-lang.org/cargo/reference/manifest.html))
- [`clippy.toml`](clippy.toml) - Clippy configuration file ([Clippy](https://github.com/rust-lang/rust-clippy#configuration))
- [`rustfmt.toml`](rustfmt.toml) - rustfmt configuration file ([rustfmt](https://github.com/rust-lang/rustfmt#configuring-rustfmt))
- [`rust-toolchain`](rust-toolchain) - rustup toolchain configuration file ([rustup](https://github.com/rust-lang/rustup.rs#the-toolchain-file))

- [`.travis.yml`](.travis.yml) - travis CI configuration ([File Format](https://docs.travis-ci.com/user/tutorial/))

- [`CHANGELOG.md`](CHANGELOG.md) - version history
- [`LICENSE.md`](LICENSE.md) - license text
- [`README.md`](README.md) - common information (this file)
- [`ROADMAP.md`](ROADMAP.md) - future goals, wishlist, ideas


