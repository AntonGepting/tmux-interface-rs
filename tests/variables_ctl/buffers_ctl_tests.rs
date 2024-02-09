#[cfg(feature = "tmux_1_6")]
#[test]
fn get_buffers() {
    use tmux_interface::{BuffersCtl, DeleteBuffer, KillSession, NewSession, SetBuffer, Tmux};

    const TARGET_SESSION: &str = "get_buffers_test";
    const BUFFER_NAME: &str = "test_buffer";

    Tmux::with_command(NewSession::new().detached().session_name(TARGET_SESSION))
        .output()
        .unwrap();

    Tmux::with_command(SetBuffer::new().buffer_name(BUFFER_NAME).data(BUFFER_NAME))
        .output()
        .unwrap();

    let buffers = BuffersCtl::new().get_all().unwrap();
    let mut found = false;
    for buffer in buffers {
        if buffer.name == Some(BUFFER_NAME.to_string()) {
            found = true;
        }
    }
    assert!(found);

    Tmux::with_command(DeleteBuffer::new().buffer_name(BUFFER_NAME))
        .output()
        .unwrap();

    Tmux::with_command(KillSession::new().target_session(TARGET_SESSION))
        .output()
        .unwrap();
}
