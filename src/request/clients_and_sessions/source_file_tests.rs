#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn source_file() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux source-file [-nqv] path
        // (alias: source)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["source-file", "-n", "-q", "-v", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.source_file(Some(true), Some(true), Some(true), "1")
        .unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn source_file() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux source-file [-q] path
        // (alias: source)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["source-file", "-q", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.source_file(Some(true), "1").unwrap_err();
}
