#[test]
fn to_string() {
    use crate::TmuxCommand;

    let tmux = TmuxCommand::new();
    assert_eq!(tmux.to_string(), "tmux");
}
