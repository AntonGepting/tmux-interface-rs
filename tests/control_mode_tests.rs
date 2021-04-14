#[test]
fn control_mode() {
    use std::io::{BufRead, BufReader, Error, ErrorKind};
    use std::process::{Command, Stdio};
    use tmux_interface::control_mode::control_mode::ControlModeOutput;

    let stdout = Command::new("tmux")
        .args(&["-C", "attach", "-t", "0"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))
        .unwrap();

    let reader = BufReader::new(stdout);

    let mut cm_mode_lines = ControlModeOutput::new(reader.lines());
    for cm_mode_line in cm_mode_lines {
        //dbg!(cm_mode_line);
    }
}
