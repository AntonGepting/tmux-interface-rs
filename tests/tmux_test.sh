#!/bin/bash

echo "test sessions"
echo "========================================================================="
tmux ls -F "#{session_alerts}:#{session_attached}:#{session_activity}:#{session_created}:#{session_format}:#{session_last_attached}:#{session_group}:#{session_gropu_size}:#{session_group_list}:#{session_grouped}:#{session_id}:#{session_many_attached}:#{session_name}:#{session_stack}:#{session_windows}"

echo "test windows"
echo "========================================================================="
tmux lsw -a -F "#{window_activity}'#{window_activity_flag}'#{window_active}'#{window_bell_flag}'#{window_bigger}'#{window_end_flag}'#{window_flags}'#{window_format}'#{window_height}'#{window_id}'#{window_index}'#{window_last_flag}'#{window_layout}'#{window_linked}'#{window_name}'#{window_offset_x}'#{window_offset_y}'#{window_panes}'#{window_silence_flag}'#{window_stack_index}'#{window_start_flag}'#{window_visible_layout}'#{window_width}'#{window_zoomed_flag}"


echo "test panes"
echo "========================================================================="
tmux lsp -a -F "#{pane_active}'#{pane_at_bottom}'#{pane_at_left}'#{pane_at_right}'#{pane_at_top}'#{pane_bottom}'#{pane_current_command}'#{pane_current_path}'#{pane_dead}'#{pane_dead_status}'#{pane_format}'#{pane_height}'#{pane_id}'#{pane_in_mode}'#{pane_input_off}'#{pane_index}'#{pane_left}'#{pane_mode}'#{pane_pid}'#{pane_pipe}'#{pane_right}'#{pane_search_string}'#{pane_start_command}'#{pane_synchronized}'#{pane_tabs}'#{pane_title}'#{pane_top}'#{pane_tty}'#{pane_width}"
