

#[test]
fn send_keys() {
    use crate::tmux_interface::TmuxInterface;
    use crate::SendKeys;

    let tmux = TmuxInterface::new();
    let send_keys = SendKeys {
        target_pane: Some("0:1.0"),
        keys: vec!["top", "C-m"],
        ..Default::default()
    };
    tmux.send_keys(&send_keys).unwrap();
}
