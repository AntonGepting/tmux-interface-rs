#[test]
fn set_buffer() {
    use crate::SetBuffer;
    use std::borrow::Cow;

    // Set the contents of the specified buffer to data.
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // set-buffer [-aw] [-b buffer-name] [-t target-client] [-n new-buffer-name] data
    // (alias: setb)
    // ```
    //
    //
    // tmux ^2.0:
    // ```text
    // set-buffer [-a] [-b buffer-name] [-n new-buffer-name] data
    // (alias: setb)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // set-buffer [-b buffer-index] data
    // (alias: setb)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // set-buffer [-b buffer-index] [-t target-session] data
    // (alias: setb)
    // ```
    let set_buffer = SetBuffer::new();
    #[cfg(feature = "tmux_2_0")]
    let set_buffer = set_buffer.append();
    #[cfg(feature = "tmux_3_2")]
    let set_buffer = set_buffer.send_to_clipboard();
    #[cfg(feature = "tmux_2_0")]
    let set_buffer = set_buffer.buffer_name("1");
    #[cfg(feature = "tmux_3_2")]
    let set_buffer = set_buffer.target_client("2");
    #[cfg(feature = "tmux_2_0")]
    let set_buffer = set_buffer.new_buffer_name("3");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let set_buffer = set_buffer.buffer_index("4");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let set_buffer = set_buffer.target_session("5");
    #[cfg(feature = "tmux_0_8")]
    let set_buffer = set_buffer.data("6");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "setb";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_0")]
    s.push("-a");
    #[cfg(feature = "tmux_3_2")]
    s.push("-w");
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_0")]
    s.extend_from_slice(&["-n", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-b", "4"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("6");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let set_buffer = set_buffer.build().to_vec();

    assert_eq!(set_buffer, s);
}
