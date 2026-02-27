// auto-generated file
//

// Load the contents of the specified paste buffer from path.
//
// # Manual
//
// tmux >=3.2:
// ```text
// load-buffer [-w] [-b buffer-name] [-t target-client] path
// (alias: loadb)
// ```
//
// tmux >=2.0:
// ```text
// load-buffer [-b buffer-name] path
// (alias: loadb)
// ```
//
// tmux >=1.5:
// ```text
// load-buffer [-b buffer-index] path
// (alias: loadb)
// ```
//
// tmux >=0.8:
// ```text
// load-buffer [-b buffer-index] [-t target-session] path
// (alias: loadb)
// ```
#[test]
fn load_buffer() {
    use crate::{LoadBuffer, TargetPane};
    use std::borrow::Cow;

    let load_buffer = LoadBuffer::new();
    // `[-w]`
    #[cfg(feature = "tmux_3_2")]
    let load_buffer = load_buffer.send_to_clipboard();

    // `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let load_buffer = load_buffer.buffer_index("1");

    // `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    let load_buffer = load_buffer.buffer_name("2");

    // `[-t target-client]`
    #[cfg(feature = "tmux_3_2")]
    let load_buffer = load_buffer.target_client("3");

    // `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let load_buffer = load_buffer.target_session("4");

    // `[path]`
    #[cfg(feature = "tmux_0_8")]
    let load_buffer = load_buffer.path("5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "load-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "loadb";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    v.push("-w");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    v.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_2_0")]
    v.extend_from_slice(&["-b", "2"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-t", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("5");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let load_buffer = load_buffer.build().to_vec();

    assert_eq!(load_buffer, v);
}
