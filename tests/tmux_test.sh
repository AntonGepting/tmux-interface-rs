#!/bin/bash

echo "test sessions"
echo "========================================================================="
tmux ls -F "#{session_alerts}:#{session_attached}:#{session_activity}:#{session_created}:#{session_format}:#{session_last_attached}:#{session_group}:#{session_gropu_size}:#{session_group_list}:#{session_grouped}:#{session_id}:#{session_many_attached}:#{session_name}:#{session_stack}:#{session_windows}"

echo "test windows"
echo "========================================================================="
tmux lsw -a -F "#{window_activity} #{window_activity_flag} #{window_active} #{window_bell_flag} #{window_id} #{window_index} #{window_name} #{window_panes}"

echo "test panes"
echo "========================================================================="
tmux lsp -a -F "#{pane_active} #{pane_at_bottom} #{pane_at_left} #{pane_at_right} #{pane_at_top} #{pane_current_command} #{pane_id} #{pane_index}"
