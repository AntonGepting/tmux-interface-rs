use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

#[test]
fn control_mode() {
    let stdout = Command::new("tmux")
        .args(&["-C", "attach", "-t", "0"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))
        .unwrap();

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));
}
