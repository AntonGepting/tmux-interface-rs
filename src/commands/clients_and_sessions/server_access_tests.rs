#[test]
fn server_access() {
    use crate::ServerAccess;
    use std::borrow::Cow;

    // Execute commands from path
    //
    // # Manual
    //
    // tmux ^3.3:
    // ```text
    // server-access [-adlrw] [user]
    // (alias: source)
    // ```
    let server_access = ServerAccess::new();
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access.add();
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access.remove();
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access.list();
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access.read();
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access.write();
    #[cfg(feature = "tmux_3_3")]
    let source_file = source_file.user("1");

    let cmd = "server-access";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_3")]
    s.push("-a");
    #[cfg(feature = "tmux_3_3")]
    s.push("-d");
    #[cfg(feature = "tmux_3_3")]
    s.push("-l");
    #[cfg(feature = "tmux_3_3")]
    s.push("-r");
    #[cfg(feature = "tmux_3_3")]
    s.push("-w");
    #[cfg(feature = "tmux_3_3")]
    s.push("1");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let server_access = server_access.build().to_vec();

    assert_eq!(server_access, s);
}
