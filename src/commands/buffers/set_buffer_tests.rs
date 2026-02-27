// auto-generated file
//

// Set the contents of the specified buffer to data.
//
// # Manual
//
// tmux >=3.2:
// ```text
// set-buffer [-aw] [-b buffer-name] [-t target-client] [-n new-buffer-name] data
// (alias: setb)
// ```
//
// tmux >=2.0:
// ```text
// set-buffer [-a] [-b buffer-name] [-n new-buffer-name] data
// (alias: setb)
// ```
//
// tmux >=1.5:
// ```text
// set-buffer [-b buffer-index] data
// (alias: setb)
// ```
//
// tmux >=0.8:
// ```text
// set-buffer [-b buffer-index] [-t target-session] data
// (alias: setb)
// ```
#[test]
fn set_buffer() {
    use crate::{SetBuffer, TargetPane};
    use std::borrow::Cow;

    let target_pane = TargetPane::Raw("5").to_string();

    let set_buffer = SetBuffer::new();
    // `[-a]`
    #[cfg(feature = "tmux_2_0")]
    let set_buffer = set_buffer.append();

    // `[-w]`
    #[cfg(feature = "tmux_3_2")]
    let set_buffer = set_buffer.send_to_clipboard();

    // `[-b buffer-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_0")))]
    let set_buffer = set_buffer.buffer_index("1");

    // `[-b buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    let set_buffer = set_buffer.buffer_name("2");

    // `[-t target-client]`
    #[cfg(feature = "tmux_3_2")]
    let set_buffer = set_buffer.target_client("3");

    // `[-t target-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let set_buffer = set_buffer.target_session("4");

    // `[-n new-buffer-name]`
    #[cfg(feature = "tmux_2_0")]
    let set_buffer = set_buffer.new_buffer_name("5");

    // `[data]`
    #[cfg(feature = "tmux_0_8")]
    let set_buffer = set_buffer.data("6");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "setb";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_0")]
    v.push("-a");
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
    #[cfg(feature = "tmux_2_0")]
    v.extend_from_slice(&["-n", "5"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("6");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let set_buffer = set_buffer.build().to_vec();

    assert_eq!(set_buffer, v);
}
