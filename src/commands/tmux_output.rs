use std::process::ExitStatus;
use std::process::Output;

// XXX: pub inner?
#[derive(Debug, Clone)]
pub struct TmuxOutput(pub Output);

impl TmuxOutput {
    pub fn into_inner(self) -> Output {
        self.0
    }

    // from std::process::Output
    pub fn status(&self) -> ExitStatus {
        self.0.status
    }

    // from std::process::Output
    pub fn stdout(self) -> Vec<u8> {
        self.0.stdout
    }

    // from std::process::Output
    pub fn stderr(self) -> Vec<u8> {
        self.0.stderr
    }

    // from std::process::Output
    pub fn success(&self) -> bool {
        self.0.status.success()
    }

    // from std::process::Output
    pub fn code(&self) -> Option<i32> {
        self.0.status.code()
    }

    // XXX: check
    pub fn to_string(&self) -> String {
        let s = &self.0.stdout.as_slice();
        String::from_utf8_lossy(s).to_string()
    }
}
