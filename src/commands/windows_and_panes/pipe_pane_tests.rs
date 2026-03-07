// auto-generated file
//

// Pipe output sent by the program in target-pane to a shell command or vice versa
//
// # Manual
//
// tmux >=2.7:
// ```text
// pipe-pane [-IOo] [-t target-pane] [shell-command]
// (alias: pipep)
// ```
//
// tmux >=1.5:
// ```text
// pipe-pane [-o] [-t target-pane] [shell-command]
// (alias: pipep)
// ```
#[test]
fn pipe_pane() {
    use crate::PipePane;
    use std::borrow::Cow;

    let pipe_pane = PipePane::new();
    // `[-I]`
    #[cfg(feature = "tmux_2_7")]
    let pipe_pane = pipe_pane.stdout();

    // `[-O]`
    #[cfg(feature = "tmux_2_7")]
    let pipe_pane = pipe_pane.stdin();

    // `[-o]`
    #[cfg(feature = "tmux_1_5")]
    let pipe_pane = pipe_pane.open();

    // `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    let pipe_pane = pipe_pane.target_pane("1");

    // `[shell-command]`
    #[cfg(feature = "tmux_1_5")]
    let pipe_pane = pipe_pane.shell_command("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "pipe-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "pipep";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_7")]
    v.push("-I");
    #[cfg(feature = "tmux_2_7")]
    v.push("-O");
    #[cfg(feature = "tmux_1_5")]
    v.push("-o");
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_5")]
    v.push("2");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let pipe_pane = pipe_pane.build().to_vec();

    assert_eq!(pipe_pane, v);
}
