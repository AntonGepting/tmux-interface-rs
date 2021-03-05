use std::process::Output;

// XXX: pub inner?
#[derive(Debug)]
pub struct TmuxOutput(pub Output);
