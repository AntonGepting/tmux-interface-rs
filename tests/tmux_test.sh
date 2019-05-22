#!/bin/bash

echo "test sessions"
echo "========================================================================="
tmux ls -F "#{session_attached} #{session_activity} #{session_created} #{session_last_attached} #{session_id} #{session_name} #{session_windows}"
echo "test windows"
echo "========================================================================="
tmux lsw -a -F "#{window_activity} #{window_activity_flag} #{window_active} #{window_bell_flag} #{window_id} #{window_index} #{window_name} #{window_panes}"
echo "test panes"
echo "========================================================================="
tmux lsp -a -F "#{pane_active} #{pane_at_bottom} #{pane_at_left} #{pane_at_right} #{pane_at_top} #{pane_current_command} #{pane_id} #{pane_index}"
