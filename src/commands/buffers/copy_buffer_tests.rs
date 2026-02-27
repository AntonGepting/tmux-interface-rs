// auto-generated file
//

#[test]
fn copy_buffer() {
    use crate::{CopyBuffer, TargetPane};
    use std::borrow::Cow;

    let target_pane = TargetPane::Raw("5").to_string();

    let copy_buffer = CopyBuffer::new();
    // `[-a src-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let copy_buffer = copy_buffer.src_index("1");

    // `[-b dst-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let copy_buffer = copy_buffer.dst_index("2");

    // `[-s src-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let copy_buffer = copy_buffer.src_session("3");

    // `[-t dst-session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let copy_buffer = copy_buffer.dst_session("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "copy-buffer";
    #[cfg(feature = "cmd_alias")]
    let cmd = "copyb";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-a", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-b", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-s", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "4"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let copy_buffer = copy_buffer.build().to_vec();

    assert_eq!(copy_buffer, v);
}
