#[test]
fn wait_for() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux wait-for [-L | -S | -U] channel
        // (alias: wait)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["wait-for", "-L", "-S", "-U", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.wait_for(Some(true), Some(true), Some(true), "1")
        .unwrap_err();
}
