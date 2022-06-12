#[test]
fn load_buffer() {
    use crate::LoadBuffer;
    use std::borrow::Cow;

    // Load the contents of the specified paste buffer from path.
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // tmux load-buffer [-w] [-b buffer-name] [-t target-client] path
    // (alias: loadb)
    // ```
    //
    // tmux ^2.0:
    // ```text
    // tmux load-buffer [-b buffer-name] path
    // (alias: loadb)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux load-buffer [-b buffer-index] path
    // (alias: loadb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux load-buffer [-b buffer-index] [-t target-session] path
    // (alias: loadb)
    // ```
    let load_buffer = LoadBuffer::new();
    #[cfg(feature = "tmux_3_2")]
    let load_buffer = load_buffer.send_to_clipboard();
    #[cfg(feature = "tmux_2_0")]
    let load_buffer = load_buffer.buffer_name("1");
    #[cfg(feature = "tmux_3_2")]
    let load_buffer = load_buffer.target_client("2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let load_buffer = load_buffer.buffer_index("3");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let load_buffer = load_buffer.target_session("4");
    #[cfg(feature = "tmux_0_8")]
    let load_buffer = load_buffer.path("5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "load-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "loadb";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-w");
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("5");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let load_buffer = load_buffer.build().to_vec();

    assert_eq!(load_buffer, s);
}
