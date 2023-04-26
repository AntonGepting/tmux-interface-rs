#[test]
fn example_5_1_1() {
    use std::process::Command;

    // tmux -2 -uv new-session -ADEd -s example_5_1_1
    let output = Command::new("tmux")
        .args(["-2", "-uv", "new-session", "-ADEd", "-s", "example_5_1_1"])
        .output()
        .unwrap();

    assert!(output.status.success());

    // tmux -2 -uv kill-session -t example_5_1_1
    let output = Command::new("tmux")
        .args(["-2", "-uv", "kill-session", "-t", "example_5_1_1"])
        .output()
        .unwrap();

    assert!(output.status.success());
}
