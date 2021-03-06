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

        - [`error.rs`](src/error.rs) - error propagating functions

        - [`lib.rs`](src/lib.rs) - main library file

        - [`tmux_interface.rs`](src/tmux_interface.rs) - common functions
        - [`tmux_interface_tests.rs`](src/tmux_interface_tests.rs)

        - [`version.rs`](src/version.rs) - parse version response
        - [`version_tests.rs`](src/version_tests.rs)

    2. TMUX Options [`src/options`](src/options/)
        - [`mod.rs`](src/options/mod.rs) - module file

        - [`pane_options.rs`](src/options/pane_options.rs) - pane options
        - [`pane_options_tests.rs`](src/options/pane_options_tests.rs) - pane options tests

        - [`server_options.rs`](src/options/server_options.rs) - server options
        - [`server_options_tests.rs`](src/options/pane_options_tests.rs) - server options tests

        - [`session_options.rs`](src/options/session_options.rs) - session options
        - [`session_options_tests.rs`](src/options/session_options_tests.rs) - session options tests

        - [`window_options.rs`](src/options/window_options.rs) - window options
        - [`window_options_tests.rs`](src/options/window_options_tests.rs) - window options tests


    3. TMUX Commands (structure similar to [TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html)):

        - [`src/commands/buffers`](src/commands/buffers/) ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#BUFFERS))
            - [`mod.rs`](src/commands/buffers/mod.rs)

            - [`choose_buffer.rs`](src/commands/buffers/choose_buffer.rs) - (`^1.3`)
            - [`choose_buffer_tests.rs`](src/commands/buffers/choose_buffer_tests.rs) - (`^1.3`)

            - [`clear_history.rs`](src/commands/buffers/clear_history.rs) - (`^0.9`)
            - [`clear_history_tests.rs`](src/commands/buffers/clear_history_tests.rs) - (`^0.9`)

            - [`delete_buffer.rs`](src/commands/buffers/delete_buffer.rs) - (`^0.8`)
            - [`delete_buffer_tests.rs`](src/commands/buffers/delete_buffer_tests.rs) - (`^0.8`)

            - [`list_buffers.rs`](src/commands/buffers/list_buffers.rs) - (`^0.8`)
            - [`list_buffers_tests.rs`](src/commands/buffers/list_buffers_tests.rs) - (`^0.8`)

            - [`load_buffer.rs`](src/commands/buffers/load_buffer.rs) - (`^0.8`)
            - [`load_buffer_tests.rs`](src/commands/buffers/load_buffer_tests.rs) - (`^0.8`)

            - [`paste_buffer.rs`](src/commands/buffers/paste_buffer.rs) - (`^0.8`)
            - [`paste_buffer_tests.rs`](src/commands/buffers/paste_buffer_tests.rs) - (`^0.8`)

            - [`save_buffer.rs`](src/commands/buffers/save_buffer.rs) - (`^0.8`)
            - [`save_buffer_tests.rs`](src/commands/buffers/save_buffer_tests.rs) - (`^0.8`)

            - [`set_buffer.rs`](src/commands/buffers/set_buffer.rs) - (`^0.8`)
            - [`set_buffer_tests.rs`](src/commands/buffers/set_buffer_tests.rs) - (`^0.8`)

            - [`show_buffer.rs`](src/commands/buffers/show_buffer.rs) - (`^0.8`)
            - [`show_buffer_tests.rs`](src/commands/buffers/show_buffer_tests.rs) - (`^0.8`)

        - [`src/commands/clients_and_sessions`](src/commands/clients_and_sessions/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS))

            - [`attach_session.rs`](src/commands/clients_and_sessions/attach_session.rs) - (`^0_8`)
            - [`attach_session_tests.rs`](src/commands/clients_and_sessions/attach_session_tests.rs) - (`^0_8`)

            - [`detach_client.rs`](src/commands/clients_and_sessions/detach_client.rs) - (`^0.8`)
            - [`detach_client_tests.rs`](src/commands/clients_and_sessions/detach_client_tests.rs) - (`^0.8`)

            - [`has_session.rs`](src/commands/clients_and_sessions/has_session.rs) - (`^0.8`)
            - [`has_session_tests.rs`](src/commands/clients_and_sessions/has_session_tests.rs) - (`^0.8`)

            - [`kill_server.rs`](src/commands/clients_and_sessions/kill_server.rs) - (`^0.8`)
            - [`kill_server_tests.rs`](src/commands/clients_and_sessions/kill_server_tests.rs) - (`^0.8`)

            - [`kill_session.rs`](src/commands/clients_and_sessions/kill_session.rs) - (`^0.8`)
            - [`kill_session_tests.rs`](src/commands/clients_and_sessions/kill_session_tests.rs) - (`^0.8`)

            - [`list_clients.rs`](src/commands/clients_and_sessions/list_clients.rs) - (`^0.8`)
            - [`list_clients_tests.rs`](src/commands/clients_and_sessions/list_clients_tests.rs) - (`^0.8`)

            - [`list_commands.rs`](src/commands/clients_and_sessions/list_commands.rs) - (`^0.8`)
            - [`list_commands_tests.rs`](src/commands/clients_and_sessions/list_commands_tests.rs) - (`^0.8`)

            - [`list_sessions.rs`](src/commands/clients_and_sessions/list_sessions.rs) - (`^0.8`)
            - [`list_sessions_tests.rs`](src/commands/clients_and_sessions/list_sessions_tests.rs) - (`^0.8`)

            - [`lock_client.rs`](src/commands/clients_and_sessions/lock_client.rs) - (`^1.1`)
            - [`lock_client_tests.rs`](src/commands/clients_and_sessions/lock_client_tests.rs) - (`^1.1`)

            - [`lock_session.rs`](src/commands/clients_and_sessions/lock_session.rs) - (`^1.1`)
            - [`lock_session_tests.rs`](src/commands/clients_and_sessions/lock_session_tests.rs) - (`^1.1`)

            - [`mod.rs`](src/commands/clients_and_sessions/mod.rs)

            - [`new_session.rs`](src/commands/clients_and_sessions/new_session.rs) - (`^0.8`)
            - [`new_session_tests.rs`](src/commands/clients_and_sessions/new_session_tests.rs) - (`^0.8`)

            - [`refresh_client.rs`](src/commands/clients_and_sessions/refresh_client.rs) - (`^0.8`)
            - [`refresh_client_tests.rs`](src/commands/clients_and_sessions/refresh_client_tests.rs) - (`^0.8`)

            - [`rename_session.rs`](src/commands/clients_and_sessions/rename_session.rs) - (`^0.8`)
            - [`rename_session_tests.rs`](src/commands/clients_and_sessions/rename_session_tests.rs) - (`^0.8`)

            - [`show_messages.rs`](src/commands/clients_and_sessions/show_messages.rs) - (`^1.2`)
            - [`show_messages_tests.rs`](src/commands/clients_and_sessions/show_messages_tests.rs) - (`^1.2`)

            - [`source_file.rs`](src/commands/clients_and_sessions/source_file.rs) - (`^0.8`)
            - [`source_file_tests.rs`](src/commands/clients_and_sessions/source_file_tests.rs) - (`^0.8`)

            - [`start_server.rs`](src/commands/clients_and_sessions/start_server.rs) - (`^0.8`)
            - [`start_server_tests.rs`](src/commands/clients_and_sessions/start_server_tests.rs) - (`^0.8`)

            - [`suspend_client.rs`](src/commands/clients_and_sessions/suspend_client.rs) - (`^0.8`)
            - [`suspend_client_tests.rs`](src/commands/clients_and_sessions/suspend_client_tests.rs) - (`^0.8`)

            - [`switch_client.rs`](src/commands/clients_and_sessions/switch_client.rs) - (`^0.8`)
            - [`switch_client_tests.rs`](src/commands/clients_and_sessions/switch_client_tests.rs) - (`^0.8`)

        - [`src/commands/global_and_session_environment`](src/commands/global_and_session_environment/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#GLOBAL_AND_SESSION_ENVIRONMENT))

            - [`mod.rs`](src/commands/global_and_session_environment/mod.rs)

            - [`set_environment.rs`](src/commands/global_and_session_environment/set_environment.rs) - (`^1.0`)
            - [`set_environment_tests.rs`](src/commands/global_and_session_environment/set_environment_tests.rs) - (`^1.0`)

            - [`show_environment.rs`](src/commands/global_and_session_environment/show_environment.rs) - (`^1.0`)
            - [`show_environment_tests.rs`](src/commands/global_and_session_environment/show_environment_tests.rs) - (`^1.0`)

        - [`src/commands/hooks`](src/commands/hooks/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#HOOKS))

            - [`mod.rs`](src/commands/hooks/mod.rs)

            - [`set_hook.rs`](src/commands/hooks/set_hook.rs) - (`^2.2`)
            - [`set_hook_tests.rs`](src/commands/hooks/set_hook_tests.rs) - (`^2.2`)

            - [`show_hooks.rs`](src/commands/hooks/show_hooks.rs) - (`^2.2`)
            - [`show_hooks_tests.rs`](src/commands/hooks/show_hooks_tests.rs) - (`^2.2`)

        - [`src/commands/key_bindings`](src/commands/key_bindings/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#KEY_BINDINGS))


            - [`bind_key.rs`](src/commands/key_bindings/bind_key.rs) - (`^0.8`)
            - [`bind_key_tests.rs`](src/commands/key_bindings/bind_key_tests.rs) - (`^0.8`)

            - [`list_keys.rs`](src/commands/key_bindings/list_keys.rs) - (`^0.8`)
            - [`list_keys_tests.rs`](src/commands/key_bindings/list_keys_tests.rs) - (`^0.8`)

            - [`mod.rs`](src/command/key_bindings/mod.rs)

            - [`send_keys.rs`](src/command/key_bindings/send_keys.rs) - (`^0.8`)
            - [`send_keys_tests.rs`](src/command/key_bindings/send_keys_tests.rs) - (`^0.8`)

            - [`send_prefix.rs`](src/command/key_bindings/send_prefix.rs) - (`^0.8`)
            - [`send_prefix_tests.rs`](src/command/key_bindings/send_prefix_tests.rs) - (`^0.8`)

            - [`unbind_key.rs`](src/command/key_bindings/unbind_key.rs) - (`^0.8`)
            - [`unbind_key_tests.rs`](src/command/key_bindings/unbind_key_tests.rs) - (`^0.8`)


        - [`src/command/miscellaneous`](src/command/miscellaneous/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#MISCELLANEOUS))


            - [`clock_mode.rs`](src/command/miscellaneous/clock_mode.rs) - (`^0.8`)
            - [`clock_mode_tests.rs`](src/command/miscellaneous/clock_mode_tests.rs) - (`^0.8`)

            - [`if_shell.rs`](src/command/miscellaneous/if_shell.rs) - (`^0.8`)
            - [`if_shell_tests.rs`](src/command/miscellaneous/if_shell_tests.rs) - (`^0.8`)

            - [`lock_server.rs`](src/command/miscellaneous/lock_server.rs) - (`^0.8`)
            - [`lock_server_tests.rs`](src/command/miscellaneous/lock_server_tests.rs) - (`^0.8`)

            - [`mod.rs`](src/command/miscellaneous/mod.rs)

            - [`run_shell.rs`](src/command/miscellaneous/run_shell.rs) - (`^1.1`)
            - [`run_shell_tests.rs`](src/command/miscellaneous/run_shell_tests.rs) - (`^1.1`)

            - [`wait_for.rs`](src/command/miscellaneous/wait_for.rs) - (`^1.8`)
            - [`wait_for_tests.rs`](src/command/miscellaneous/wait_for_tests.rs) - (`^1.8`)

        - [`src/command/options`](src/command/options/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#OPTIONS))

            - [`mod.rs`](src/command/options/mod.rs)

            - [`set_option.rs`](src/command/options/set_option.rs) - (`^0.8`)
            - [`set_option_tests.rs`](src/command/options/set_option_tests.rs) - (`^0.8`)

            - [`set_window_option.rs`](src/command/options/set_window_option.rs) - (`^0.8`)
            - [`set_window_option_tests.rs`](src/command/options/set_window_option_tests.rs) - (`^0.8`)

            - [`show_options.rs`](src/command/options/show_options.rs) - (`^0.8`)
            - [`show_options_tests.rs`](src/command/options/show_options_tests.rs) - (`^0.8`)

            - [`show_window_options.rs`](src/command/options/show_window_options.rs) - (`^0.8`)
            - [`show_window_options_tests.rs`](src/command/options/show_window_options_tests.rs) - (`^0.8`)

        - [`src/command/status_line`](src/command/status_line/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#STATUS_LINE))

            - [`command_prompt.rs`](src/command/status_line/command_prompt.rs) - (`^0.8`)
            - [`command_prompt_tests.rs`](src/command/status_line/command_prompt_tests.rs) - (`^0.8`)

            - [`confirm_before.rs`](src/command/status_line/confirm_before.rs) - (`^0.9`)
            - [`confirm_before_tests.rs`](src/command/status_line/confirm_before_tests.rs) - (`^0.9`)

            - [`display_menu.rs`](src/command/status_line/display_menu.rs) - (`^3.0`)
            - [`display_menu_tests.rs`](src/command/status_line/display_menu_tests.rs) - (`^3.0`)

            - [`display_message.rs`](src/command/status_line/display_message.rs) - (`^1.0`)
            - [`display_message_tests.rs`](src/command/status_line/display_message_tests.rs) - (`^1.0`)

            - [`mod.rs`](src/command/status_line/mod.rs)

        - [`src/command/windows_and_panes`](src/command/windows_and_panes/) - ([TMUX manual](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES))


            - [`break_pane.rs`](src/command/windows_and_panes/break_pane.rs) - (`^0.8`)
            - [`break_pane_tests.rs`](src/command/windows_and_panes/break_pane_tests.rs) - (`^0.8`)

            - [`capture_pane.rs`](src/command/windows_and_panes/capture_pane.rs) - (`^1.2`)
            - [`capture_pane_tests.rs`](src/command/windows_and_panes/capture_pane_tests.rs) - (`^1.2`)

            - [`choose_client.rs`](src/command/windows_and_panes/choose_client.rs) - (`^1.0`)
            - [`choose_client_tests.rs`](src/command/windows_and_panes/choose_client_tests.rs) - (`^1.0`)

            - [`choose_tree.rs`](src/command/windows_and_panes/choose_tree.rs) - (`^1.7`)
            - [`choose_tree_tests.rs`](src/command/windows_and_panes/choose_tree_tests.rs) - (`^1.7`)

            - [`copy_mode.rs`](src/command/windows_and_panes/copy_mode.rs) - (`^0.8`)
            - [`copy_mode_tests.rs`](src/command/windows_and_panes/copy_mode_tests.rs) - (`^0.8`)

            - [`display_pane.rs`](src/command/windows_and_panes/display_pane.rs) - (`^1.0`)
            - [`display_pane_tests.rs`](src/command/windows_and_panes/display_pane_tests.rs) - (`^1.0`)

            - [`find_window.rs`](src/command/windows_and_panes/find_window.rs) - (`^0.8`)
            - [`find_window_tests.rs`](src/command/windows_and_panes/find_window_tests.rs) - (`^0.8`)

            - [`join_pane.rs`](src/command/windows_and_panes/join_pane.rs) - (`^1.2`)
            - [`join_pane_tests.rs`](src/command/windows_and_panes/join_pane_tests.rs) - (`^1.2`)

            - [`kill_pane.rs`](src/command/windows_and_panes/kill_pane.rs) - (`^0.8`)
            - [`kill_pane_tests.rs`](src/command/windows_and_panes/kill_pane_tests.rs) - (`^0.8`)

            - [`kill_window.rs`](src/command/windows_and_panes/kill_window.rs) - (`^0.8`)
            - [`kill_window_tests.rs`](src/command/windows_and_panes/kill_window_tests.rs) - (`^0.8`)

            - [`last_pane.rs`](src/command/windows_and_panes/last_pane.rs) - (`^1.4`)
            - [`last_pane_tests.rs`](src/command/windows_and_panes/last_pane_tests.rs) - (`^1.4`)

            - [`last_window.rs`](src/command/windows_and_panes/last_window.rs) - (`^0.8`)
            - [`last_window_tests.rs`](src/command/windows_and_panes/last_window_tests.rs) - (`^0.8`)

            - [`link_window.rs`](src/command/windows_and_panes/link_window.rs) - (`^0.8`)
            - [`link_window_tests.rs`](src/command/windows_and_panes/link_window_tests.rs) - (`^0.8`)

            - [`list_panes.rs`](src/command/windows_and_panes/list_panes.rs) - (`^0.8`)
            - [`list_panes_tests.rs`](src/command/windows_and_panes/list_panes_tests.rs) - (`^0.8`)

            - [`list_windows.rs`](src/command/windows_and_panes/list_windows.rs) - (`^0.8`)
            - [`list_windows_tests.rs`](src/command/windows_and_panes/list_windows_tests.rs) - (`^0.8`)

            - [`mod.rs`](src/command/windows_and_panes/mod.rs)

            - [`move_pane.rs`](src/command/windows_and_panes/move_pane.rs) - (`^1.7`)
            - [`move_pane_tests.rs`](src/command/windows_and_panes/move_pane_tests.rs) - (`^1.7`)

            - [`move_window.rs`](src/command/windows_and_panes/move_window.rs) - (`^0.8`)
            - [`move_window_tests.rs`](src/command/windows_and_panes/move_window_tests.rs) - (`^0.8`)

            - [`new_window.rs`](src/command/windows_and_panes/new_window.rs) - (`^0.8`)
            - [`new_window_tests.rs`](src/command/windows_and_panes/new_window_tests.rs) - (`^0.8`)

            - [`next_layout.rs`](src/command/windows_and_panes/next_layout.rs) - (`^0.8`)
            - [`next_layout_tests.rs`](src/command/windows_and_panes/next_layout_tests.rs) - (`^0.8`)

            - [`next_window.rs`](src/command/windows_and_panes/next_window.rs) - (`^0.8`)
            - [`next_window_tests.rs`](src/command/windows_and_panes/next_window_tests.rs) - (`^0.8`)

            - [`pipe_pane.rs`](src/command/windows_and_panes/pipe_pane.rs) - (`^1.1`)
            - [`pipe_pane_tests.rs`](src/command/windows_and_panes/pipe_pane_tests.rs) - (`^1.1`)

            - [`previous_layout.rs`](src/command/windows_and_panes/previous_layout.rs) - (`^1.3`)
            - [`previous_layout_tests.rs`](src/command/windows_and_panes/previous_layout_tests.rs) - (`^1.3`)

            - [`previous_window.rs`](src/command/windows_and_panes/previous_window.rs) - (`^0.8`)
            - [`previous_window_tests.rs`](src/command/windows_and_panes/previous_window_tests.rs) - (`^0.8`)

            - [`rename_window.rs`](src/command/windows_and_panes/rename_window.rs) - (`^0.8`)
            - [`rename_window_tests.rs`](src/command/windows_and_panes/rename_window_tests.rs) - (`^0.8`)

            - [`resize_pane.rs`](src/command/windows_and_panes/resize_pane.rs) - (`^0.9`)
            - [`resize_pane_tests.rs`](src/command/windows_and_panes/resize_pane_tests.rs) - (`^0.9`)

            - [`resize_window.rs`](src/command/windows_and_panes/resize_window.rs) - (`^2.9`)
            - [`resize_window_tests.rs`](src/command/windows_and_panes/resize_window_tests.rs) - (`^2.9`)

            - [`respawn_pane.rs`](src/command/windows_and_panes/respawn_pane.rs) - (`^1.5`)
            - [`respawn_pane_tests.rs`](src/command/windows_and_panes/respawn_pane_tests.rs) - (`^1.5`)

            - [`respawn_window.rs`](src/command/windows_and_panes/respawn_window.rs) - (`^0.8`)
            - [`respawn_window_tests.rs`](src/command/windows_and_panes/respawn_window_tests.rs) - (`^0.8`)

            - [`rotate_window.rs`](src/command/windows_and_panes/rotate_window.rs) - (`^0.8`)
            - [`rotate_window_tests.rs`](src/command/windows_and_panes/rotate_window_tests.rs) - (`^0.8`)

            - [`select_layout.rs`](src/command/windows_and_panes/select_layout.rs) - (`^0.9`)
            - [`select_layout_tests.rs`](src/command/windows_and_panes/select_layout_tests.rs) - (`^0.9`)

            - [`select_pane.rs`](src/command/windows_and_panes/select_pane.rs) - (`^0.8`)
            - [`select_pane_tests.rs`](src/command/windows_and_panes/select_pane_tests.rs) - (`^0.8`)

            - [`select_window.rs`](src/command/windows_and_panes/select_window.rs) - (`^0.8`)
            - [`select_window_tests.rs`](src/command/windows_and_panes/select_window_tests.rs) - (`^0.8`)

            - [`split_window.rs`](src/command/windows_and_panes/split_window.rs) - (`^0.8`)
            - [`split_window_tests.rs`](src/command/windows_and_panes/split_window_tests.rs) - (`^0.8`)

            - [`swap_pane.rs`](src/command/windows_and_panes/swap_pane.rs) - (`^0.8`)
            - [`swap_pane_tests.rs`](src/command/windows_and_panes/swap_pane_tests.rs) - (`^0.8`)

            - [`swap_window.rs`](src/command/windows_and_panes/swap_window.rs) - (`^0.8`)
            - [`swap_window_tests.rs`](src/command/windows_and_panes/swap_window_tests.rs) - (`^0.8`)

            - [`unlink_window.rs`](src/command/windows_and_panes/unlink_window.rs) - (`^0.8`)
            - [`unlink_window_tests.rs`](src/command/windows_and_panes/unlink_window_tests.rs) - (`^0.8`)

        - [`mod.rs`](src/command/mod.rs)

    4. TMUX Target module

        - [`target_pane.rs`](src/target/target_pane.rs) - TargetPane & TargetPaneEx objects
        - [`target_pane_tests.rs`](src/target/target_pane_tests.rs)

        - [`target_session.rs`](src/target/target_session.rs) - TargetSession object
        - [`target_session_tests.rs`](src/target/target_session_tests.rs)

        - [`target_window.rs`](src/target/target_window.rs) - TargetWindow & TargetWindowEx objects
        - [`target_window_tests.rs`](src/target/target_window_tests.rs)


    4. TMUX Variables parsing functions

        - [`src/variables`](src/variables)
            - [`mod.rs`](src/variables/mod.rs)

        - [`src/variables/layout`](src/variables/layout)
            - [`mod.rs`](src/variables/layout/mod.rs)

            - [`layout_cell.rs`](src/layout_cell.rs) - parse layout cell string
            - [`layout_cell_tests.rs`](src/layout_cell_tests.rs)

            - [`layout_checksum.rs`](src/layout_checksum.rs) - calculate layout checksum
            - [`layout_checksum_tests.rs`](src/layout_checksum_tests.rs)

            - [`layout.rs`](src/layout.rs) - parse layot tree string
            - [`layout_tests.rs`](src/layout_tests.rs)


        - [`src/variables/session`](src/variables/session)
            - [`mod.rs`](src/variables/session/mod.rs)

            - [`session.rs`](src/session.rs) - parse a session
            - [`session_tests.rs`](src/session_tests.rs)

            - [`sessions.rs`](src/sessions.rs) - parse a session list
            - [`sessions_tests.rs`](src/sessions_tests.rs)

            - [`session_stack.rs`](src/session_stack.rs) - session stack


        - [`src/variables/window`](src/variables/window)
            - [`mod.rs`](src/variables/window/mod.rs)

            - [`window_flag.rs`](src/window_flag.rs) - window flag

            - [`window.rs`](src/window.rs) - parse a window
            - [`window_tests.rs`](src/window_tests.rs)

            - [`windows.rs`](src/windows.rs) - parse a windows list
            - [`windows_tests.rs`](src/windows_tests.rs)


        - [`src/variables/pane`](src/variables/pane)
            - [`mod.rs`](src/variables/pane/mod.rs)

            - [`pane.rs`](src/variables/pane.rs) - parse a pane
            - [`pane_tests.rs`](src/variables/pane_tests.rs)

            - [`panes.rs`](src/variables/panes.rs) - parse a panes list
            - [`panes_tests.rs`](src/variables/panes_tests.rs)

            - [`pane_tabs.rs`](src/variables/pane_tabs.rs) - pane tabs


- [`tests/`](tests/) - crate integration tests (multiple functions):

    - [`issue1.rs`](tests/issue1.rs) - issue #1 tests
    - [`sessions_tests.rs`](tests/sessions_tests.rs) - sessions tests
    - [`windows_tests.rs`](tests/windows_tests.rs) - windows tests
    - [`panes_tests.rs`](tests/panes_tests.rs) - panes tests
    - [`tmux_interface.rs`](tests/tmux_interface.rs)

- [`scripts/`](scripts/) - scripts and variables:
    - [`man_tmux/`](man_tmux/) scripts for tmux manual analysis:
        - [`build_all.sh`](scripts/man_tmux/build_all.sh) - bash script for building library for all tmux versions
        - [`generate_manpages.sh`](scripts/man_tmux/generate_manpages.sh) - bash script for generating man pages for all tmux versions
        - [`grep_options.sh`](scripts/man_tmux/grep_options.sh) - bash script for extracting tmux options from man pages
        - [`grep_variables.sh`](scripts/man_tmux/grep_variables.sh) - bash script for extracting tmux variables from man pages
        - [`versions.var`](scripts/man_tmux/versions.var) - contains tmux versions variables for using in scripts
    - [`tmux_error_mock.sh`](tests/tmux_error_mock.sh) - bash script for testing of tmux error handling functions
    - [`tmux_mock.sh`](tests/tmux_mock.sh) - bash script can be used instead of tmux binary, for simple logging
        (sniffing) intercommmunication between library functions and tmux
    - [`tmux_test.sh`](tests/tmux_test.sh) - bash script for output testing of tmux functions
    - [`tmux_variables_test.py`](tests/tmux_variables_test.py) - bash script for output testing of tmux functions

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

