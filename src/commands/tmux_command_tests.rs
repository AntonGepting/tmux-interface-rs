#[test]
fn to_string() {
    use crate::{Error, TmuxCommand, TmuxOutput};

    let mut tmux = TmuxCommand::new();
    assert_eq!(tmux.to_string(), "tmux");
}
