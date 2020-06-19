#[test]
fn kill_server() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux kill-server
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["kill-server", ""]"#
        );
        Err(Error::Hook)
    }));
    tmux.kill_server().unwrap_err();
}
