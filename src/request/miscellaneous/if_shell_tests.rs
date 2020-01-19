#[test]
fn if_shell() {
    use crate::{Error, IfShell, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux if-shell [-bF] [-t target-pane] shell-command command [command]
        // (alias: if)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["if-shell", "-b", "-F", "-t", "1", "2", "3", "4"]"#
        );
        Err(Error::new("hook"))
    }));
    let if_shell = IfShell {
        backgroud: Some(true),
        not_execute: Some(true),
        target_pane: Some("1"),
        second_command: Some("4"),
    };
    tmux.if_shell(Some(&if_shell), "2", "3").unwrap_err();
}
