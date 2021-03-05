#[test]
fn lock_session() {
    use crate::{Error, LockSession, TargetSession, TmuxCommand, TmuxInterface};

    // tmux lock-session [-t target-session]
    // (alias: locks)
    let mut lock_session = LockSession::new();
    lock_session.target_session("1");

    let mut s = Vec::new();
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();
    let tmux_command: TmuxCommand = lock_session.into();

    #[cfg(not(feature = "use_cmd_alias"))]
    let cmd = "lock-session";
    #[cfg(feature = "use_cmd_alias")]
    let cmd = "locks";

    assert_eq!(tmux_command.bin, None);
    assert_eq!(tmux_command.bin_args, None);
    assert_eq!(tmux_command.cmd, Some(cmd.into()));
    assert_eq!(tmux_command.cmd_args, Some(s));
}
