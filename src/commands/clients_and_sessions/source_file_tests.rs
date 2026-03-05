// auto-generated file
//

// Execute commands from path
//
// # Manual
//
// tmux >=3.4:
// ```text
// source-file [-Fnqv] [-t target-pane] path ...
// (alias: source)
// ```
//
// tmux >=3.2:
// ```text
// source-file [-Fnqv] path ...
// (alias: source)
// ```
//
// tmux >=3.0a:
// ```text
// source-file [-nqv] path
// (alias: source)
// ```
//
// tmux >=3.0:
// ```text
// source-file [-nq] path
// (alias: source)
// ```
//
// tmux >=2.3:
// ```text
// source-file [-q] path
// (alias: source)
//
// ```
// tmux >=0.8:
// ```text
// source-file path
// (alias: source)
// ```
#[test]
fn source_file() {
    use crate::SourceFile;
    use std::borrow::Cow;

    let source_file = SourceFile::new();
    // `[-F]`
    #[cfg(feature = "tmux_3_2")]
    let source_file = source_file.expand();

    // `[-n]`
    #[cfg(feature = "tmux_3_0")]
    let source_file = source_file.not_exclude();

    // `[-q]`
    #[cfg(feature = "tmux_2_3")]
    let source_file = source_file.quiet();

    // `[-v]`
    #[cfg(feature = "tmux_3_0a")]
    let source_file = source_file.verbose();

    // `[-t target-pane]`
    #[cfg(feature = "tmux_3_4")]
    let source_file = source_file.target_pane("1");

    // `[path]`
    #[cfg(feature = "tmux_0_8")]
    let source_file = source_file.path("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "source-file";
    #[cfg(feature = "cmd_alias")]
    let cmd = "source";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    v.push("-F");
    #[cfg(feature = "tmux_3_0")]
    v.push("-n");
    #[cfg(feature = "tmux_2_3")]
    v.push("-q");
    #[cfg(feature = "tmux_3_0a")]
    v.push("-v");
    #[cfg(feature = "tmux_3_4")]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("2");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let source_file = source_file.build().to_vec();

    assert_eq!(source_file, v);
}
