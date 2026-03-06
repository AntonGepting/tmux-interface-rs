// auto-generated file
//

// Show server information
//
// tmux >=0.8 && <=1.9:
// ```text
// server-info
// (alias: info)
// ```
#[test]
fn server_info() {
    use crate::ServerInfo;
    use std::borrow::Cow;

    let server_info = ServerInfo::new();

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "server-info";
    #[cfg(feature = "cmd_alias")]
    let cmd = "info";

    let mut v = Vec::new();
    v.push(cmd);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let server_info = server_info.build().to_vec();

    assert_eq!(server_info, v);
}
