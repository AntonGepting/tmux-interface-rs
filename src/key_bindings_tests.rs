

#[test]
fn send_keys() {
    use crate::tmux_interface::TmuxInterface;
    use crate::SendKeys;
    use std::borrow::Cow;

    let tmux = TmuxInterface::new();
    let send_keys = SendKeys {
        target_pane: Some(Cow::Borrowed("0:1.0")),
        keys: vec!["top", "C-m"],
        ..Default::default()
    };
    tmux.send_keys(&send_keys).unwrap();
}
