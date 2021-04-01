#[test]
fn source_file() {
    use crate::SourceFile;
    use std::borrow::Cow;

    // Execute commands from path
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // tmux source-file [-nqv] path
    // (alias: source)
    // ```
    //
    // tmux ^2.3:
    // ```text
    // tmux source-file path
    // (alias: source)
    //
    // ```
    // tmux ^0.8:
    // ```text
    // tmux source-file path
    // (alias: source)
    // ```
    let mut source_file = SourceFile::new();
    #[cfg(feature = "tmux_3_0")]
    source_file.not_execute();
    #[cfg(feature = "tmux_3_0")]
    source_file.quite();
    #[cfg(feature = "tmux_3_0")]
    source_file.verbose();
    #[cfg(feature = "tmux_0_8")]
    source_file.path("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "source-file";
    #[cfg(feature = "cmd_alias")]
    let cmd = "source";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_3_0")]
    s.push("-n");
    #[cfg(feature = "tmux_3_0")]
    s.push("-q");
    #[cfg(feature = "tmux_3_0")]
    s.push("-v");
    #[cfg(feature = "tmux_0_8")]
    s.push("1");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(source_file.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(source_file.0.bin_args, None);
    assert_eq!(source_file.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(source_file.0.cmd_args, Some(s));
}
