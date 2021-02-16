#!/bin/bash

echo "tmux list-sessions -F \"...\""
echo "========================================================================="
tmux ls -F "#{session_activity}:#{session_alerts}:#{session_attached}:\
#{session_created}:#{session_format}:#{session_group}:#{session_group_list}:\
#{session_group_size}:#{session_grouped}:#{session_height}:#{session_width}:\
#{session_id}:#{session_last_attached}:#{session_many_attached}:#{session_name}:\
#{session_stack}:#{session_windows}:\
#{test_wrong_variable}"
echo "========================================================================="


echo "tmux list-windows -a -F \"...\""
echo "========================================================================="
tmux lsw -a -F "#{window_active}'#{window_activity}'#{window_activity_flag}'\
#{window_bell_flag}'#{window_bigger}'#{window_end_flag}'#{window_flags}'\
#{window_format}'#{window_height}'#{window_id}'#{window_index}'\
#{window_last_flag}'#{window_layout}'#{window_linked}'#{window_name}'\
#{window_offset_x}'#{window_offset_y}'#{window_panes}'#{window_silence_flag}'\
#{window_stack_index}'#{window_start_flag}'#{window_visible_layout}'\
#{window_width}'#{window_zoomed_flag}'\
#{test_wrong_variable}"
echo "========================================================================="

echo "tmux list-panes -a -F \"...\""
echo "========================================================================="
tmux lsp -a -F "#{pane_active}'#{pane_at_bottom}'#{pane_at_left}'\
#{pane_at_right}'#{pane_at_top}'#{pane_bottom}'#{pane_current_command}'\
#{pane_current_path}'#{pane_dead}'#{pane_dead_status}'#{pane_format}'\
#{pane_height}'#{pane_id}'#{pane_in_mode}'#{pane_index}'#{pane_input_off}'\
#{pane_left}'#{pane_marked}'#{pane_marked_set}'#{pane_mode}'#{pane_pid}'\
#{pane_pipe}'#{pane_right}'#{pane_search_string}'#{pane_start_command}'\
#{pane_synchronized}'#{pane_tabs}'#{pane_title}'#{pane_top}'#{pane_tty}'\
#{pane_width}'\
#{test_wrong_variable}"
echo "========================================================================="

echo "tmux ls -F \"from all objects...\""
echo "========================================================================="
tmux ls -F "#{client_pid}:#{client_name}:#{session_activity}:#{window_active}:#{pane_active}:\
#{buffer_name}:#{command}:#{cursor_x}\
#{test_wrong_variable}"
echo "========================================================================="

echo "tmux litst-client -F \"...\""
echo "========================================================================="
tmux lsc -F "#{client_pid}:#{client_name}:#{session_activity}:#{window_active}:#{pane_active}:\
#{buffer_name}:#{command}:#{cursor_x}\
#{test_wrong_variable}"
echo "========================================================================="

# array with all formats
formats=( \
"alternate_on" \
"alternate_saved_x" \
"alternate_saved_y" \
"buffer_created" \
"buffer_name" \
"buffer_sample" \
"buffer_size" \
"client_activity" \
"client_cell_height" \
"client_cell_width" \
"client_control_mode" \
"client_created" \
"client_discarded" \
"client_height" \
"client_key_table" \
"client_last_session" \
"client_name" \
"client_pid" \
"client_prefix" \
"client_readonly" \
"client_session" \
"client_termname" \
"client_tty" \
"client_utf8" \
"client_width" \
"client_written" \
"command" \
"command_list_alias" \
"command_list_name" \
"command_list_usage" \
"copy_cursor_line" \
"copy_cursor_word" \
"copy_cursor_x" \
"copy_cursor_y" \
"cursor_character" \
"cursor_flag" \
"cursor_x" \
"cursor_y" \
"history_bytes" \
"history_limit" \
"history_size" \
"hook" \
"hook_pane" \
"hook_session" \
"hook_session_name" \
"hook_window" \
"hook_window_name" \
"host" \
"host_short" \
"insert_flag" \
"keypad_cursor_flag" \
"keypad_flag" \
"line" \
"mouse_all_flag" \
"mouse_any_flag" \
"mouse_button_flag" \
"mouse_line" \
"mouse_sgr_flag" \
"mouse_standard_flag" \
"mouse_utf8_flag" \
"mouse_word" \
"mouse_x" \
"mouse_y" \
"origin_flag" \
"pane_active" \
"pane_at_bottom" \
"pane_at_left" \
"pane_at_right" \
"pane_at_top" \
"pane_bottom" \
"pane_current_command" \
"pane_current_path" \
"pane_dead" \
"pane_dead_status" \
"pane_format" \
"pane_height" \
"pane_id" \
"pane_in_mode" \
"pane_index" \
"pane_input_off" \
"pane_left" \
"pane_marked" \
"pane_marked_set" \
"pane_mode" \
"pane_path" \
"pane_pid" \
"pane_pipe" \
"pane_right" \
"pane_search_string" \
"pane_start_command" \
"pane_synchronized" \
"pane_tabs" \
"pane_title" \
"pane_top" \
"pane_tty" \
"pane_width" \
"pid" \
"rectangle_toggle" \
"scroll_position" \
"scroll_region_lower" \
"scroll_region_upper" \
"selection_end_x" \
"selection_end_y" \
"selection_present" \
"selection_start_x" \
"selection_start_y" \
"session_activity" \
"session_alerts" \
"session_attached" \
"session_attached_list" \
"session_created" \
"session_format" \
"session_group" \
"session_group_attached" \
"session_group_attached_list" \
"session_group_list" \
"session_group_many_attached" \
"session_group_size" \
"session_grouped" \
"session_id" \
"session_last_attached" \
"session_many_attached" \
"session_name" \
"session_stack" \
"session_windows" \
"socket_path" \
"start_time" \
"version" \
"window_active" \
"window_active_clients" \
"window_active_clients_list" \
"window_active_sessions" \
"window_active_sessions_list" \
"window_activity" \
"window_activity_flag" \
"window_bell_flag" \
"window_bigger" \
"window_cell_height" \
"window_cell_width" \
"window_end_flag" \
"window_flags" \
"window_format" \
"window_height" \
"window_id" \
"window_index" \
"window_last_flag" \
"window_layout" \
"window_linked" \
"window_linked_sessions" \
"window_linked_sessions_list" \
"window_marked_flag" \
"window_name" \
"window_offset_x" \
"window_offset_y" \
"window_panes" \
"window_silence_flag" \
"window_stack_index" \
"window_start_flag" \
"window_visible_layout" \
"window_width" \
"window_zoomed_flag" \
"wrap_flag" \
)


# all formats as string
for a in ${formats[@]}; do
    all_formats+="#{$a}:"
done

# test tmux commands with all formats string
# result: formats are command specific, grouping needed
# https://leanpub.com/the-tao-of-tmux/read#appendix-formats
echo "tmux list-clients -F \"\$all_formats\""
echo "========================================================================="
tmux lsc -F $all_formats
echo "========================================================================="

echo "tmux list-sessions -F \"\$all_formats\""
echo "========================================================================="
tmux ls -F $all_formats
echo "========================================================================="

echo "tmux list-windows -F \"\$all_formats\""
echo "========================================================================="
tmux lsw -F $all_formats
echo "========================================================================="

echo "tmux list-panes -F \"\$all_formats\""
echo "========================================================================="
tmux lsp -F $all_formats
echo "========================================================================="
