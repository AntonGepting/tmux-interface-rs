// auto-generated file
//

//  Change the access or read/write permission of user
//
// # Manual
//
// tmux >=3.3:
// ```text
// server-access [-adlrw] [user]
// ```
#[test]
fn server_access() {
    use crate::ServerAccess;
    use std::borrow::Cow;

    let server_access = ServerAccess::new();
    // `[-a]`
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access.add();

    // `[-d]`
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access.delete();

    // `[-l]`
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access.list();

    // `[-r]`
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access.read();

    // `[-w]`
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access.write();

    // `[user]`
    #[cfg(feature = "tmux_3_3")]
    let server_access = server_access.user("");

    let cmd = "server-access";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_3")]
    v.push("-a");
    #[cfg(feature = "tmux_3_3")]
    v.push("-d");
    #[cfg(feature = "tmux_3_3")]
    v.push("-l");
    #[cfg(feature = "tmux_3_3")]
    v.push("-r");
    #[cfg(feature = "tmux_3_3")]
    v.push("-w");
    #[cfg(feature = "tmux_3_3")]
    v.push("");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let server_access = server_access.build().to_vec();

    assert_eq!(server_access, v);
}
