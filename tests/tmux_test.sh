#!/bin/bash

echo "tmux ls -F \"...\""
echo "========================================================================="
tmux ls -F "#{session_activity}:#{session_alerts}:#{session_attached}:\
#{session_created}:#{session_format}:#{session_group}:#{session_group_list}:\
#{session_group_size}:#{session_grouped}:#{session_id}:\
#{session_last_attached}:#{session_many_attached}:#{session_name}:\
#{session_stack}:#{session_windows}"


echo "tmux list-windows -a -F \"...\""
echo "========================================================================="
tmux lsw -a -F "#{window_active}'#{window_activity}'#{window_activity_flag}'\
#{window_bell_flag}'#{window_bigger}'#{window_end_flag}'#{window_flags}'\
#{window_format}'#{window_height}'#{window_id}'#{window_index}'\
#{window_last_flag}'#{window_layout}'#{window_linked}'#{window_name}'\
#{window_offset_x}'#{window_offset_y}'#{window_panes}'#{window_silence_flag}'\
#{window_stack_index}'#{window_start_flag}'#{window_visible_layout}'\
#{window_width}'#{window_zoomed_flag}"


echo "tmux list-panes -a -F \"...\""
echo "========================================================================="
tmux lsp -a -F "#{pane_active}'#{pane_at_bottom}'#{pane_at_left}'\
#{pane_at_right}'#{pane_at_top}'#{pane_bottom}'#{pane_current_command}'\
#{pane_current_path}'#{pane_dead}'#{pane_dead_status}'#{pane_format}'\
#{pane_height}'#{pane_id}'#{pane_in_mode}'#{pane_index}'#{pane_input_off}'\
#{pane_left}'#{pane_marked}'#{pane_marked_set}'#{pane_mode}'#{pane_pid}'\
#{pane_pipe}'#{pane_right}'#{pane_search_string}'#{pane_start_command}'\
#{pane_synchronized}'#{pane_tabs}'#{pane_title}'#{pane_top}'#{pane_tty}'\
#{pane_width}"
