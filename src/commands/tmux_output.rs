use std::fmt;
use std::process::ExitStatus;
use std::process::Output;

// XXX: pub inner?
#[derive(Clone, Debug)]
pub struct TmuxOutput(pub Output);

// XXX: check (impl used instead of method to_string)
impl fmt::Display for TmuxOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = &self.0.stdout.as_slice();
        let output = String::from_utf8_lossy(s).to_string();
        write!(f, "{}", output)
    }
}

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
}
