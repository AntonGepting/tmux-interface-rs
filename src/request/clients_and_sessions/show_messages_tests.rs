#[test]
fn show_messages() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux show-messages [-JT] [-t target-client]
        // (alias: showmsgs)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["show-messages", "-J", "-T", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.show_messages(Some(true), Some(true), Some("1"))
        .unwrap_err();
}
