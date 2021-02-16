#!/bin/python3

# TODO: base on sources
# - book - [book](https://leanpub.com/the-tao-of-tmux/read#appendix-formats)
# - default - sources / formats.c default functions
#
class Formats:
    formats = [
        [ "Pane", "alternate_on" ],                     # book
        [ "Pane", "alternate_saved_x" ],                # book
        [ "Pane", "alternate_saved_y" ],                # book
        [ "Buffer", "buffer_created" ],                 #     , default
        [ "Buffer", "buffer_name" ],                    # book, default
        [ "Buffer", "buffer_sample" ],                  # book, default
        [ "Buffer", "buffer_size" ],                    # book, default
        [ "Client", "client_activity" ],                # book
        [ "Client", "client_cell_height" ],             # book
        [ "Client", "client_cell_width" ],
        [ "Client", "client_control_mode" ],            # book
        [ "Client", "client_created" ],                 # book
        [ "Client", "client_discarded" ],
        [ "Client", "client_height" ],
        [ "Client", "client_key_table" ],               # book
        [ "Client", "client_last_session" ],            # book
        [ "Client", "client_name" ],
        [ "Client", "client_pid" ],                     # book
        [ "Client", "client_prefix" ],                  # book
        [ "Client", "client_readonly" ],                # book
        [ "Client", "client_session" ],                 # book
        [ "Client", "client_termname" ],                # book
        [ "Client", "client_tty" ],                     # book
        [ "Client", "client_utf8" ],                    # book
        [ "Client", "client_width" ],                   # book
        [ "Client", "client_written" ],
        [ "Command", "command" ],                       # default cmd-list-keys
        [ "Command", "command_list_alias" ],            # default cmd-list-keys
        [ "Command", "command_list_name" ],             # default cmd-list-keys
        [ "Command", "command_list_usage" ],            # default cmd-list-keys
        [ "Window", "copy_cursor_line" ],               # default window copy
        [ "Window", "copy_cursor_word" ],               # default window copy
        [ "Window", "copy_cursor_x" ],                  # default window copy
        [ "Window", "copy_cursor_y" ],                  # default window copy
        [ "Pane", "cursor_character" ],                 #     , default
        [ "Pane", "cursor_flag" ],                      # book, default
        [ "Pane", "cursor_x" ],                         # book, default
        [ "Pane", "cursor_y" ],                         # book, default
        [ "Window", "history_bytes" ],                  # Window book, Pane default
        [ "Window", "history_limit" ],                  # Window book, Pane default
        [ "Window", "history_size" ],                   # Window book, Pane default
        [ "Hook", "hook" ],                             # default notify
        [ "Hook", "hook_pane" ],                        # default notify
        [ "Hook", "hook_session" ],                     # default notify
        [ "Hook", "hook_session_name" ],                # default notify
        [ "Hook", "hook_window" ],                      # default notify
        [ "Hook", "hook_window_name" ],                 # default notify
        [ "Server", "host" ],                           # book
        [ "Server", "host_short" ],                     # book
        [ "Pane", "insert_flag" ],                      # book, default
        [ "Pane", "keypad_cursor_flag" ],               # book, default
        [ "Pane", "keypad_flag" ],                      # book, default
        [ "Client", "line" ],                           # book
        [ "Pane", "mouse_all_flag" ],                   # book, default
        [ "Pane", "mouse_any_flag" ],                   # book, default
        [ "Pane", "mouse_button_flag" ],                # book, default
        [ "Server", "mouse_line" ],                     # default mb server
        [ "Pane", "mouse_sgr_flag" ],                   # default
        [ "Pane", "mouse_standard_flag" ],              # book, default
        [ "Pane", "mouse_utf8_flag" ],                  # default mb server
        [ "Server", "mouse_word" ],                     # default mb server
        [ "Server", "mouse_x" ],                        # default mb server
        [ "Server", "mouse_y" ],                        # default mb server
        [ "Pane", "origin_flag" ],                      # default
        [ "Pane", "pane_active" ],                      # book
        [ "Pane", "pane_at_bottom" ],                   # book
        [ "Pane", "pane_at_left" ],
        [ "Pane", "pane_at_right" ],
        [ "Pane", "pane_at_top" ],
        [ "Pane", "pane_bottom" ],
        [ "Pane", "pane_current_command" ],             # book
        [ "Pane", "pane_current_path" ],                # book
        [ "Pane", "pane_dead" ],                        # book
        [ "Pane", "pane_dead_status" ],                 # book
        [ "Pane", "pane_format" ],
        [ "Pane", "pane_height" ],                      # book
        [ "Pane", "pane_id" ],                          # book
        [ "Pane", "pane_in_mode" ],                     # book
        [ "Pane", "pane_index" ],                       # book
        [ "Pane", "pane_input_off" ],                   # book
        [ "Pane", "pane_left" ],                        # book
        [ "Pane", "pane_marked" ],
        [ "Pane", "pane_marked_set" ],
        [ "Pane", "pane_mode" ],
        [ "Pane", "pane_path" ],
        [ "Pane", "pane_pid" ],                         # book
        [ "Pane", "pane_pipe" ],
        [ "Pane", "pane_right" ],                       # book
        [ "Pane", "pane_search_string" ],
        [ "Pane", "pane_start_command" ],               # book
        [ "Pane", "pane_synchronized" ],                # book
        [ "Pane", "pane_tabs" ],                        # book
        [ "Pane", "pane_title" ],                       # book
        [ "Pane", "pane_top" ],                         # book
        [ "Pane", "pane_tty" ],                         # book
        [ "Pane", "pane_width" ],                       # book
        [ "Server", "pid" ],                            # book
        [ "Window", "rectangle_toggle" ],               # default window copy
        [ "Pane", "scroll_position" ],                  # book
        [ "Pane", "scroll_region_lower" ],              # book, default
        [ "Pane", "scroll_region_upper" ],              # book, default
        [ "Window", "selection_end_x" ],                # default window copy
        [ "Window", "selection_end_y" ],                # default window copy
        [ "Window", "selection_present" ],              # default window copy
        [ "Window", "selection_start_x" ],              # default window copy
        [ "Window", "selection_start_y" ],              # default window copy
        [ "Session", "session_activity" ],              # book
        [ "Session", "session_alerts" ],                # book
        [ "Session", "session_attached" ],              # book
        [ "Session", "session_attached_list" ],
        [ "Session", "session_created" ],               # book
        [ "Session", "session_format" ],
        [ "Session", "session_group" ],                 # book
        [ "Session", "session_group_attached" ],
        [ "Session", "session_group_attached_list" ],
        [ "Session", "session_group_list" ],
        [ "Session", "session_group_many_attached" ],   # book
        [ "Session", "session_group_size" ],
        [ "Session", "session_grouped" ],               # book
        # session_height missing
        # book session_width missing
        [ "Session", "session_id" ],                    # book
        [ "Session", "session_last_attached" ],         # book
        [ "Session", "session_many_attached" ],
        [ "Session", "session_name" ],                  # book
        [ "Session", "session_stack" ],
        [ "Session", "session_windows" ],               # book
        [ "Server", "socket_path" ],                    # book
        [ "Server", "start_time" ],                     # book
        [ "Server", "version" ],                        # book
        [ "Window", "window_active" ],                  # book
        [ "Window", "window_active_clients" ],
        [ "Window", "window_active_clients_list" ],
        [ "Window", "window_active_sessions" ],
        [ "Window", "window_active_sessions_list" ],
        [ "Window", "window_activity" ],                # book
        [ "Window", "window_activity_flag" ],           # book
        [ "Window", "window_bell_flag" ],               # book
        [ "Window", "window_bigger" ],
        [ "Window", "window_cell_height" ],
        [ "Window", "window_cell_width" ],
        [ "Window", "window_end_flag" ],
        [ "Window", "window_flags" ],                   # book
        # book window_find_matches missing
        [ "Window", "window_format" ],
        [ "Window", "window_height" ],                  # book
        [ "Window", "window_id" ],                      # book
        [ "Window", "window_index" ],                   # book
        [ "Window", "window_last_flag" ],               # book
        [ "Window", "window_layout" ],                  # book
        [ "Window", "window_linked" ],                  # book
        [ "Window", "window_linked_sessions" ],
        [ "Window", "window_linked_sessions_list" ],
        [ "Window", "window_marked_flag" ],
        [ "Window", "window_name" ],                    # book
        [ "Window", "window_offset_x" ],
        [ "Window", "window_offset_y" ],
        [ "Window", "window_panes" ],                   # book
        [ "Window", "window_silence_flag" ],            # book
        [ "Window", "window_stack_index" ],
        [ "Window", "window_start_flag" ],
        [ "Window", "window_visible_layout" ],          # book
        [ "Window", "window_width" ],                   # book
        [ "Window", "window_zoomed_flag" ],             # book
        [ "Pane", "wrap_flag" ],                        # book, default
    ]

    # get string with all formats of given group
    def get(self, group):
        l = ""
        for row in self.formats:
            if row[0] == group:
                l += "{0:30}: #{{{1}}}\n".format(row[1], row[1])
        return l

    def get_session(self):
        return self.get("Session")

    def get_buffer(self):
        return self.get("Buffer")

    def get_window(self):
        return self.get("Window")

    def get_client(self):
        return self.get("Client")

    def get_pane(self):
        return self.get("Pane")

    def get_server(self):
        return self.get("Server")

    def get_all(self):
        l = ""
        for row in self.formats:
            l += "{0:30}: #{{{1}}}\n".format(row[1], row[1])
        return l

# print(get("Session"))

def print_for_formats(formats):
    import os

    print("===================================================================")
    print("list-buffers")
    print("===================================================================")
    os.system('tmux list-buffers -F "{}"'.format(formats))
    print("===================================================================")
    print("list-commands")
    print("===================================================================")
    os.system('tmux list-commands -F "{}"'.format(formats))
    print("===================================================================")
    print("list-clients")
    print("===================================================================")
    os.system('tmux list-clients -F "{}"'.format(formats))
    print("===================================================================")
    print("list-sessions")
    print("===================================================================")
    os.system('tmux list-sessions -F "{}"'.format(formats))
    print("===================================================================")
    print("list-windows")
    print("===================================================================")
    os.system('tmux list-windows -F "{}"'.format(formats))
    print("===================================================================")
    print("list-panes")
    print("===================================================================")
    os.system('tmux list-panes -F "{}"'.format(formats))





formats = Formats()
print("===================================================================")
print(" Buffer ")
print("===================================================================")
l = formats.get_buffer()
print_for_formats(l)
print("===================================================================")
print(" Pane ")
print("===================================================================")
l = formats.get_pane()
print_for_formats(l)
print("===================================================================")
print(" Window ")
print("===================================================================")
l = formats.get_window()
print_for_formats(l)
print("===================================================================")
print(" Session ")
print("===================================================================")
l = formats.get_session()
print_for_formats(l)
print("===================================================================")
print(" Client ")
print("===================================================================")
l = formats.get_client()
print_for_formats(l)
print("===================================================================")
print(" Server ")
print("===================================================================")
l = formats.get_server()
print_for_formats(l)
print("===================================================================")
print(" All ")
print("===================================================================")
l = formats.get_all()
print_for_formats(l)

