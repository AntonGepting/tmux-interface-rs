use crate::tmux_interface::*;
use crate::{TmuxCommand, TmuxCommandTrait};

impl Tmux for TmuxCommand {}

// XXX: using environment vars
pub trait Tmux: TmuxCommandTrait {
    const TMUX: &'static str = "tmux";

    fn new() -> TmuxCommand {
        TmuxCommand {
            bin: Some(<TmuxCommand as Tmux>::TMUX.to_string()),
            bin_args: None,
            cmd: None,
            cmd_args: None,
        }
    }

    //fn bin<S: Into<Cow<'a, str>>>(mut self, bin: S) -> Self {
    //self.bin = Some(bin.into());
    //self
    //}

    fn version(&mut self) -> &mut Self {
        self.push_flag(V_KEY);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    fn colours256(&mut self) -> &mut Self {
        self.push_flag(_2_KEY);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    fn control_mode(&mut self) -> &mut Self {
        self.push_flag(C_KEY);
        self
    }

    fn disable_echo(&mut self) -> &mut Self {
        self.push_flag(CC_KEY);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    fn login_shell(&mut self) -> &mut Self {
        self.push_flag(l_KEY);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    fn force_utf8(&mut self) -> &mut Self {
        self.push_flag(u_KEY);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    fn verbose_logging(&mut self) -> &mut Self {
        self.push_flag(v_KEY);
        self
    }

    #[cfg(feature = "tmux_1_1")]
    fn shell_cmd<S: Into<String>>(&mut self, shell_cmd: S) -> &mut Self {
        self.push_option(c_KEY, shell_cmd);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    fn file<S: Into<String>>(&mut self, file: S) -> &mut Self {
        self.push_option(f_KEY, file);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    fn socket_name<S: Into<String>>(&mut self, socket_name: S) -> &mut Self {
        self.push_option(L_KEY, socket_name);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    fn socket_path<S: Into<String>>(&mut self, socket_path: S) -> &mut Self {
        self.push_option(S_KEY, socket_path);
        self
    }

    // XXX: ???
    //pub fn cmd(self) -> TmuxCommand {
    //self.0
    //}

    //pub fn owned(self) -> Tmux {
    //self
    //}

    //pub fn new_session(&self) -> NewSession {
    //}

    //fn exec(&self) -> TmuxOutput {
    //self.exec()
    //}

    //fn build(self) -> Self {
    //self
    //}
}

//impl From<Tmux> for TmuxCommand {
//fn from(item: Tmux) -> Self {
//item.0
//}
//}

//impl From<&Tmux> for TmuxCommand {
//fn from(item: &Tmux) -> Self {
//item.0.clone()
//}
//}

//impl MyCommand for Tmux {
//fn set_bin<S: Into<Cow<'a, str>>>(&mut self, bin: S) -> &mut Self {
//self.0.bin = Some(bin.into());
//self
//}

//fn get_bin(&self) -> Option<&Cow<'a, str>> {
//self.0.bin.as_ref()
//}
//}
