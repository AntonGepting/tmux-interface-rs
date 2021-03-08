#[test]
fn has_session() {
    use crate::{Error, TargetSession, TmuxInterface};

    // tmux ^0.8:
    // ```text
    // tmux has-session [-t target-session]
    // (alias: has)
    // ```
    let mut new_session = NewSession::new();
    new_session.target_session("1");

    let mut s = Vec::new();
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    #[cfg(not(feature = "use_cmd_alias"))]
    let cmd = "has-session";
    #[cfg(feature = "use_cmd_alias")]
    let cmd = "has";

    assert_eq!(tmux_command.bin, None);
    assert_eq!(tmux_command.bin_args, None);
    assert_eq!(tmux_command.cmd, Some(cmd.into()));
    assert_eq!(tmux_command.cmd_args, Some(s));
}
