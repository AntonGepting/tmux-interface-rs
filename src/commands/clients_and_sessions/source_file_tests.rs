#[test]
fn source_file() {
    use crate::SourceFile;
    use std::borrow::Cow;

    // Execute commands from path
    //
    // # Manual
    //
    // tmux ^3.4:
    // ```text
    // source-file [-Fnqv] [-t target-pane] path ...
    // (alias: source)
    // ```
    //
    // tmux ^3.2:
    // ```text
    // source-file [-Fnqv] path ...
    // (alias: source)
    // ```
    //
    // tmux ^3.0:
    // ```text
    // source-file [-nqv] path
    // (alias: source)
    // ```
    //
    // tmux ^2.3:
    // ```text
    // source-file path
    // (alias: source)
    //
    // ```
    // tmux ^0.8:
    // ```text
    // source-file path
    // (alias: source)
    // ```
    let source_file = SourceFile::new();
    #[cfg(feature = "tmux_3_2")]
    let source_file = source_file.expand();
    #[cfg(feature = "tmux_3_0")]
    let source_file = source_file.not_execute();
    #[cfg(feature = "tmux_3_0")]
    let source_file = source_file.quiet();
    #[cfg(feature = "tmux_3_0")]
    let source_file = source_file.verbose();
    #[cfg(feature = "tmux_3_4")]
    let source_file = source_file.target_pane("1");
    #[cfg(feature = "tmux_0_8")]
    let source_file = source_file.path("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "source-file";
    #[cfg(feature = "cmd_alias")]
    let cmd = "source";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-F");
    #[cfg(feature = "tmux_3_0")]
    s.push("-n");
    #[cfg(feature = "tmux_3_0")]
    s.push("-q");
    #[cfg(feature = "tmux_3_0")]
    s.push("-v");
    #[cfg(feature = "tmux_3_4")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let source_file = source_file.build().to_vec();

    assert_eq!(source_file, s);
}
