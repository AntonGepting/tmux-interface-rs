use crate::tmux_interface::*;
use crate::{TmuxCommand, TmuxCommandTrait};
use std::borrow::Cow;

impl<'a> Tmux<'a> for TmuxCommand<'a> {}

// XXX: using environment vars
pub trait Tmux<'a>: TmuxCommandTrait<'a> {
    const TMUX: &'static str = "tmux";

    fn new() -> TmuxCommand<'a> {
        TmuxCommand {
            bin: Some(Cow::Borrowed(<TmuxCommand as Tmux>::TMUX)),
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
    fn shell_cmd<S: Into<Cow<'a, str>>>(&mut self, shell_cmd: S) -> &mut Self {
        self.push_option(c_KEY, shell_cmd);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    fn file<S: Into<Cow<'a, str>>>(&mut self, file: S) -> &mut Self {
        self.push_option(f_KEY, file);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    fn socket_name<S: Into<Cow<'a, str>>>(&mut self, socket_name: S) -> &mut Self {
        self.push_option(L_KEY, socket_name);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    fn socket_path<S: Into<Cow<'a, str>>>(&mut self, socket_path: S) -> &mut Self {
        self.push_option(S_KEY, socket_path);
        self
    }

    // XXX: ???
    //pub fn cmd(self) -> TmuxCommand<'a> {
    //self.0
    //}

    //pub fn owned(self) -> Tmux<'a> {
    //self
    //}

    //pub fn new_session(&self) -> NewSession<'a> {
    //}

    //fn exec(&self) -> TmuxOutput {
    //self.exec()
    //}

    //fn build(self) -> Self {
    //self
    //}
}

//impl<'a> From<Tmux<'a>> for TmuxCommand<'a> {
//fn from(item: Tmux<'a>) -> Self {
//item.0
//}
//}

//impl<'a> From<&Tmux<'a>> for TmuxCommand<'a> {
//fn from(item: &Tmux<'a>) -> Self {
//item.0.clone()
//}
//}

//impl<'a> MyCommand<'a> for Tmux<'a> {
//fn set_bin<S: Into<Cow<'a, str>>>(&mut self, bin: S) -> &mut Self {
//self.0.bin = Some(bin.into());
//self
//}

//fn get_bin(&self) -> Option<&Cow<'a, str>> {
//self.0.bin.as_ref()
//}
//}
