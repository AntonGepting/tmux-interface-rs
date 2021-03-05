use crate::tmux_interface::*;
use crate::tmux_output::TmuxOutput;
use std::borrow::Cow;
use std::process::{Command, Output, Stdio};

// 2. String for hooks and mutability
// 1. bin and cmd must be in same struct?
//      [x] one struct, understanding~+
//      [ ] two structs, complexity~+, usability~-
//      call wrapping impossible cmd+args(tmux+args) != tmux args cmd args
// - Check tmux order options flags matters?
//
// - exec vs run
//      [x] exec - execute
//      [ ] run
//
//  - String or str in struct
//      [x] &str - chep runtime
//      [ ] String - modification
//
// - no need to sset Option<bool>
#[derive(Default, Debug, Clone)]
pub struct TmuxCommand<'a> {
    //pub tmux: Tmux<'a>,
    pub bin: Option<&'a str>,
    pub bin_args: Option<Vec<&'a str>>,
    pub cmd: Option<&'a str>,
    pub cmd_args: Option<Vec<Cow<'a, str>>>,
}

impl<'a> TmuxCommand<'a> {
    const TMUX: &'static str = "tmux";

    pub fn new() -> Self {
        TmuxCommand {
            bin: None,
            bin_args: None,
            cmd: None,
            cmd_args: None,
        }
    }

    pub fn create(
        bin: Option<&'a str>,
        bin_args: Option<Vec<&'a str>>,
        cmd: Option<&'a str>,
        cmd_args: Option<Vec<Cow<'a, str>>>,
    ) -> Self {
        TmuxCommand {
            bin: bin.clone(),
            bin_args: bin_args.clone(),
            cmd: cmd.clone(),
            cmd_args: cmd_args.clone(),
        }
    }

    pub fn bin(&mut self, bin: &'a str) -> &mut Self {
        //self.tmux.bin = bin;
        self.bin = Some(bin);
        self
    }

    pub fn cmd(&mut self, cmd: &'a str) -> &mut Self {
        self.cmd = Some(cmd);
        self
    }

    //pub fn cmd_args(&mut self, )

    //pub fn new_session(&self) -> NewSession<'a> {
    //NewSession(TmuxCommand::create(
    //self.bin,
    //self.bin_args.clone(),
    //Some("new-session"),
    //self.cmd_args.clone(),
    //))
    //}

    //pub fn kill_session(&self) -> KillSession<'a> {
    //KillSession(TmuxCommand {
    ////tmux: self.tmux.clone(),
    ////args: self.args.clone(),
    //bin: self.bin.clone(),
    //bin_args: self.bin_args.clone(),
    //cmd: Some("kill-session"),
    //cmd_args: self.cmd_args.clone(),
    //})
    //}

    //pub fn print(&self) {
    //println!("{:?}", self);
    //}

    /// insert a single flag (`-f, --flag`)
    pub fn insert_flag<C: Into<Cow<'a, str>>>(&mut self, flag: C) -> &mut Self {
        self.insert_param(flag.into())
    }

    /// insert an option (`-f, --flag <option>`)
    pub fn insert_option<C, D>(&mut self, key: C, option: D) -> &mut Self
    where
        C: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>,
    {
        self.cmd_args
            .get_or_insert(Vec::new())
            .extend_from_slice(&[key.into(), option.into()]);
        self
    }

    /// insert a single parameter (`[param]`)
    pub fn insert_param<C: Into<Cow<'a, str>>>(&mut self, param: C) -> &mut Self {
        self.cmd_args.get_or_insert(Vec::new()).push(param.into());
        self
    }

    //// NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
    //cmd.stdin(Stdio::inherit());
    pub fn exec(&mut self) -> TmuxOutput {
        let tmux_bin = &self.bin.unwrap_or(TmuxCommand::TMUX);
        let mut command = Command::new(tmux_bin);
        if let Some(s) = &self.bin_args {
            command.args(s);
        }
        if let Some(s) = &self.cmd {
            command.arg(s);
        }
        // XXX: ugly?
        if let Some(s) = &self.cmd_args {
            for a in s {
                command.arg(&**a);
            }
        }
        println!("{:?}", &self);
        command.stdin(Stdio::inherit());
        let output = command.output().unwrap();
        println!("{:?}", output);
        TmuxOutput(output)
    }
}
