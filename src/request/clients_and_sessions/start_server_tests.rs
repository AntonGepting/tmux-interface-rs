#[test]
fn start_server() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux start-server
        // (alias: start)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["start-server", ""]"#
        );
        Err(Error::Hook)
    }));
    tmux.start_server().unwrap_err();
}
