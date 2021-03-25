use std::process::ExitStatus;
use std::process::Output;

// XXX: pub inner?
#[derive(Debug, Clone)]
pub struct TmuxOutput(pub Output);

impl TmuxOutput {
    pub fn output(self) -> Output {
        self.0
    }

    pub fn status(&self) -> ExitStatus {
        self.0.status
    }

    // TODO: refactor
    pub fn string(&self) -> String {
        String::from_utf8_lossy(&self.0.stdout.as_slice()).to_string()
    }
}
