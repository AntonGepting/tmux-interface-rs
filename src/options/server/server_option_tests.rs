#[test]
fn server_option_to_string() {
    use crate::ServerOption;

    #[cfg(feature = "tmux_3_1")]
    assert_eq!(ServerOption::Backspace(None).to_string(), "backspace");
    #[cfg(feature = "tmux_1_5")]
    assert_eq!(ServerOption::BufferLimit(None).to_string(), "buffer-limit");
    #[cfg(feature = "tmux_2_4")]
    {
        assert_eq!(
            ServerOption::CommandAlias(None).to_string(),
            "command-alias"
        );
        assert_eq!(
            ServerOption::CommandAlias(Some((0, "".to_string()))).to_string(),
            "command-alias[0] "
        );
    }
    #[cfg(feature = "tmux_2_1")]
    assert_eq!(
        ServerOption::DefaultTerminal(None).to_string(),
        "default-terminal"
    );
    #[cfg(feature = "tmux_1_2")]
    assert_eq!(ServerOption::EscapeTime(None).to_string(), "escape-time");
    #[cfg(feature = "tmux_2_7")]
    assert_eq!(ServerOption::ExitEmpty(None).to_string(), "exit-empty");
    #[cfg(feature = "tmux_1_4")]
    assert_eq!(
        ServerOption::ExitUnattached(None).to_string(),
        "exit-unattached"
    );
    #[cfg(feature = "tmux_3_2")]
    assert_eq!(
        ServerOption::ExtendedKeys(None).to_string(),
        "extended-keys"
    );
    #[cfg(feature = "tmux_1_9")]
    assert_eq!(ServerOption::FocusEvents(None).to_string(), "focus-events");
    #[cfg(feature = "tmux_2_1")]
    assert_eq!(ServerOption::HistoryFile(None).to_string(), "history-file");
    #[cfg(feature = "tmux_2_0")]
    assert_eq!(
        ServerOption::MessageLimit(None).to_string(),
        "message-limit"
    );
    #[cfg(feature = "tmux_1_5")]
    assert_eq!(
        ServerOption::SetClipboard(None).to_string(),
        "set-clipboard"
    );
    //#[cfg(feature = "tmux_2_0")]
    //assert_eq!(
    //ServerOption::TerminalOverrides(None).to_string(),
    //"terminal-overrides"
    //);
    #[cfg(feature = "tmux_3_0")]
    assert_eq!(ServerOption::UserKeys(None).to_string(), "user-keys");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    assert_eq!(ServerOption::Quiet(None).to_string(), "quiet");
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    assert_eq!(
        ServerOption::DetachOnDestroy(None).to_string(),
        "detach-on-destroy"
    );
}

//#[test]
//fn parse_server_option() {
//use crate::ServerOption;

//#[cfg(feature = "tmux_3_1")]
//{
//let o = ServerOption::from("backspace C-?");
//assert_eq!(o, ServerOption::Backspace(Some("C-?".to_string())));
//}
//}
