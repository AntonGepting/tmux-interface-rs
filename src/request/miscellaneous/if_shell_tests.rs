#[test]
fn if_shell() {
    use crate::{Error, IfShell, IfShellBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.0:
        // ```text
        // tmux if-shell [-bF] [-t target-pane] shell-command command [command]
        // (alias: if)
        // ```
        //
        // tmux ^1.8:
        // ```text
        // tmux if-shell [-b] [-t target-pane] shell-command command [command]
        // (alias: if)
        // ```
        //
        // tmux ^1.6:
        // ```text
        // tmux if-shell shell-command command [command]
        // (alias: if)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux if-shell shell-command command
        // (alias: if)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("if-shell");
        #[cfg(feature = "tmux_1_8")]
        s.push("-b");
        #[cfg(feature = "tmux_2_0")]
        s.push("-F");
        #[cfg(feature = "tmux_1_8")]
        s.extend_from_slice(&["-t", "1"]);
        s.push("2");
        s.push("3");
        s.push("4");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_pane = TargetPane::Raw("1").to_string();
    let if_shell: IfShell = IfShell {
        #[cfg(feature = "tmux_1_8")]
        backgroud: Some(true),
        #[cfg(feature = "tmux_2_0")]
        not_execute: Some(true),
        #[cfg(feature = "tmux_1_8")]
        target_pane: Some(&target_pane),
        second_command: Some("4"),
    };
    tmux.if_shell(Some(&if_shell), "2", "3").unwrap_err();

    let mut builder: IfShellBuilder = IfShellBuilder::new();
    #[cfg(feature = "tmux_1_8")]
    builder.backgroud();
    #[cfg(feature = "tmux_2_0")]
    builder.not_execute();
    #[cfg(feature = "tmux_1_8")]
    builder.target_pane(&target_pane);
    builder.second_command("4");
    let if_shell = builder.build();
    tmux.if_shell(Some(&if_shell), "2", "3").unwrap_err();
}
