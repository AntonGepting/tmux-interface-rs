#[test]
fn set_server_options() {
    use crate::{SetClipboard, SetServerOptions, SetServerOptionsTrait, SetUserOptions, Switch};

    let cmd = "set -s";
    let target = ":";
    let cmd = format!("{} -t {}", cmd, target);

    let mut v = Vec::new();

    #[cfg(feature = "tmux_3_1")]
    v.push(format!("{} {} {}", cmd, "backspace", "C-?"));
    #[cfg(feature = "tmux_1_5")]
    v.push(format!("{} {} {}", cmd, "buffer-limit", "50"));
    #[cfg(feature = "tmux_2_4")]
    {
        v.push(format!(
            "{} {} {}",
            cmd, "command-alias[0]", "split-pane=split-window"
        ));
        v.push(format!(
            "{} {} {}",
            cmd, "command-alias[1]", "splitp=split-window"
        ));
    }
    #[cfg(feature = "tmux_3_2")]
    v.push(format!("{} {} {}", cmd, "copy-command", ""));
    #[cfg(feature = "tmux_2_1")]
    v.push(format!(
        "{} {} {}",
        cmd, "default-terminal", "screen-256color"
    ));
    #[cfg(feature = "tmux_1_2")]
    v.push(format!("{} {} {}", cmd, "escape-time", "500"));
    #[cfg(feature = "tmux_3_2")]
    v.push(format!("{} {} {}", cmd, "editor", "/usr/bin/vi"));
    #[cfg(feature = "tmux_2_7")]
    v.push(format!("{} {} {}", cmd, "exit-empty", "on"));
    #[cfg(feature = "tmux_1_4")]
    v.push(format!("{} {} {}", cmd, "exit-unattached", "off"));
    #[cfg(feature = "tmux_3_2")]
    v.push(format!("{} {} {}", cmd, "extended-keys", "off"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!("{} {} {}", cmd, "focus-events", "off"));
    #[cfg(feature = "tmux_2_1")]
    v.push(format!("{} {} {}", cmd, "history-file", ""));
    #[cfg(feature = "tmux_2_0")]
    v.push(format!("{} {} {}", cmd, "message-limit", "1000"));
    #[cfg(feature = "tmux_3_3")]
    v.push(format!("{} {} {}", cmd, "prompt-history-limit", "100"));
    #[cfg(feature = "tmux_1_5")]
    v.push(format!("{} {} {}", cmd, "set-clipboard", "external"));
    #[cfg(feature = "tmux_3_2")]
    {
        v.push(format!(
            "{} {} {}",
            cmd, "terminal-features[0]", "xterm*:clipboard:ccolour:cstyle:focus:title"
        ));
        v.push(format!(
            "{} {} {}",
            cmd, "terminal-features[1]", "screen*:title"
        ));
    }
    #[cfg(feature = "tmux_2_0")]
    v.push(format!("{} {} {}", cmd, "terminal-overrides", ""));
    #[cfg(feature = "tmux_3_0")]
    v.push(format!("{} {} {}", cmd, "user-keys", ""));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    v.push(format!("{} {} {}", cmd, "quiet", ""));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    v.push(format!("{} {} {}", cmd, "detach-on-destroy", ""));

    v.push(format!("{} {} {}", cmd, "@user-option-name", "value"));
    let origin = v.join(" ; ");

    let options = SetServerOptions::new();
    #[cfg(feature = "tmux_3_1")]
    let options = options.backspace(Some(target), Some("C-?"));
    #[cfg(feature = "tmux_1_5")]
    let options = options.buffer_limit(Some(target), Some(50));
    #[cfg(feature = "tmux_2_4")]
    let options = options.command_alias(
        Some(target),
        Some(vec![
            "split-pane=split-window".to_string(),
            "splitp=split-window".to_string(),
        ]),
    );
    #[cfg(feature = "tmux_3_2")]
    let options = options.copy_command(Some(target), Some(""));
    #[cfg(feature = "tmux_2_1")]
    let options = options.default_terminal(Some(target), Some("screen-256color"));
    #[cfg(feature = "tmux_1_2")]
    let options = options.escape_time(Some(target), Some(500));
    #[cfg(feature = "tmux_3_2")]
    let options = options.editor(Some(target), Some("/usr/bin/vi"));
    #[cfg(feature = "tmux_2_7")]
    let options = options.exit_empty(Some(target), Some(Switch::On));
    #[cfg(feature = "tmux_1_4")]
    let options = options.exit_unattached(Some(target), Some(Switch::Off));
    #[cfg(feature = "tmux_3_2")]
    let options = options.extended_keys(Some(target), Some(Switch::Off));
    #[cfg(feature = "tmux_1_9")]
    let options = options.focus_events(Some(target), Some(Switch::Off));
    #[cfg(feature = "tmux_2_1")]
    let options = options.history_file(Some(target), Some(""));
    #[cfg(feature = "tmux_2_0")]
    let options = options.message_limit(Some(target), Some(1000));
    #[cfg(feature = "tmux_3_3")]
    let options = options.prompt_history_limit(Some(target), Some(100));
    #[cfg(feature = "tmux_1_5")]
    let options = options.set_clipboard(Some(target), Some(SetClipboard::External));
    #[cfg(feature = "tmux_3_2")]
    let options = options.terminal_features(
        target,
        Some(vec![
            "xterm*:clipboard:ccolour:cstyle:focus:title".to_string(),
            "screen*:title".to_string(),
        ]),
    );
    #[cfg(feature = "tmux_2_0")]
    let options = options.terminal_overrides(Some(target), None::<Vec<String>>);
    #[cfg(feature = "tmux_3_0")]
    let options = options.user_keys(Some(target), None::<Vec<String>>);
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    let options = options.quiet(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    let options = options.detach_on_destroy(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    let options = options.detach_on_destroy(Some(target), Some(""));

    let options = options.user_option(Some(target), "user-option-name", Some("value".to_string()));

    let options = options.build().to_string();

    assert_eq!(origin, options);
}
