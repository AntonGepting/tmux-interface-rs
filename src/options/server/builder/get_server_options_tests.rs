#[test]
fn get_server_options() {
    use crate::{GetServerOptions, GetServerOptionsTrait, GetUserOptions};

    let cmd = "show -s";
    let target = Some(":");

    let mut v = Vec::new();

    #[cfg(feature = "tmux_3_1")]
    v.push(format!("{} {}", cmd, "backspace"));
    #[cfg(feature = "tmux_1_5")]
    v.push(format!("{} {}", cmd, "buffer-limit"));
    #[cfg(feature = "tmux_2_4")]
    v.push(format!("{} {}", cmd, "command-alias"));
    #[cfg(feature = "tmux_2_1")]
    v.push(format!("{} {}", cmd, "default-terminal"));
    #[cfg(feature = "tmux_3_2")]
    v.push(format!("{} {}", cmd, "copy-command"));
    #[cfg(feature = "tmux_3_2")]
    v.push(format!("{} {}", cmd, "editor"));
    #[cfg(feature = "tmux_1_2")]
    v.push(format!("{} {}", cmd, "escape-time"));
    #[cfg(feature = "tmux_2_7")]
    v.push(format!("{} {}", cmd, "exit-empty"));
    #[cfg(feature = "tmux_1_4")]
    v.push(format!("{} {}", cmd, "exit-unattached"));
    #[cfg(feature = "tmux_3_2")]
    v.push(format!("{} {}", cmd, "extended-keys"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!("{} {}", cmd, "focus-events"));
    #[cfg(feature = "tmux_2_1")]
    v.push(format!("{} {}", cmd, "history-file"));
    #[cfg(feature = "tmux_2_0")]
    v.push(format!("{} {}", cmd, "message-limit"));
    #[cfg(feature = "tmux_3_3")]
    v.push(format!("{} {}", cmd, "prompt-history-limit"));
    #[cfg(feature = "tmux_1_5")]
    v.push(format!("{} {}", cmd, "set-clipboard"));
    #[cfg(feature = "tmux_3_2")]
    v.push(format!("{} {}", cmd, "terminal-features"));
    #[cfg(feature = "tmux_2_0")]
    v.push(format!("{} {}", cmd, "terminal-overrides"));
    #[cfg(feature = "tmux_3_0")]
    v.push(format!("{} {}", cmd, "user-keys"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    v.push(format!("{} {}", cmd, "quiet"));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    v.push(format!("{} {}", cmd, "detach-on-destroy"));

    v.push(format!("{} {}", cmd, "@user-option-name"));
    let origin = v.join(" ; ");

    //dbg!(&options);

    let options = GetServerOptions::new();
    #[cfg(feature = "tmux_3_1")]
    let options = options.backspace(target);
    #[cfg(feature = "tmux_1_5")]
    let options = options.buffer_limit(target);
    #[cfg(feature = "tmux_2_4")]
    let options = options.command_alias(target);
    #[cfg(feature = "tmux_2_1")]
    let options = options.default_terminal(target);
    #[cfg(feature = "tmux_3_2")]
    let options = options.copy_command(target);
    #[cfg(feature = "tmux_3_2")]
    let options = options.editor(target);
    #[cfg(feature = "tmux_1_2")]
    let options = options.escape_time(target);
    #[cfg(feature = "tmux_2_7")]
    let options = options.exit_empty(target);
    #[cfg(feature = "tmux_1_4")]
    let options = options.exit_unattached(target);
    #[cfg(feature = "tmux_3_2")]
    let options = options.extended_keys(target);
    #[cfg(feature = "tmux_1_9")]
    let options = options.focus_events(target);
    #[cfg(feature = "tmux_2_1")]
    let options = options.history_file(target);
    #[cfg(feature = "tmux_2_0")]
    let options = options.message_limit(target);
    #[cfg(feature = "tmux_3_3")]
    let options = options.prompt_history_limit(target);
    #[cfg(feature = "tmux_1_5")]
    let options = options.set_clipboard(target);
    #[cfg(feature = "tmux_3_2")]
    let options = options.terminal_features(target);
    #[cfg(feature = "tmux_2_0")]
    let options = options.terminal_overrides(target);
    #[cfg(feature = "tmux_3_0")]
    let options = options.user_keys(target);
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    let options = options.quiet(target);
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    let options = options.detach_on_destroy(target);
    let options = options.user_option(target, "user-option-name");

    let options = options.build().to_string();

    assert_eq!(options, origin);
}
