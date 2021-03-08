use crate::tmux_interface::*;
use crate::{TmuxCommand, TmuxOutput};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Tmux<'a>(pub TmuxCommand<'a>);

impl<'a> Default for Tmux<'a> {
    fn default() -> Self {
        Tmux(TmuxCommand {
            bin: Some(Tmux::TMUX.into()),
            bin_args: None,
            cmd: None,
            cmd_args: None,
        })
    }
}

// XXX: using environment vars
impl<'a> Tmux<'a> {
    const TMUX: &'static str = "tmux";

    pub fn new() -> Self {
        Default::default()
    }

    pub fn bin<S: Into<Cow<'a, str>>>(mut self, bin: S) -> Self {
        self.0.bin = Some(bin.into());
        self
    }

    pub fn version(&mut self) -> &mut Self {
        self.0.push_flag(V_KEY);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn colours256(&mut self) -> &mut Self {
        self.0.push_flag(_2_KEY);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn control_mode(&mut self) -> &mut Self {
        self.0.push_flag(C_KEY);
        self
    }

    pub fn disable_echo(&mut self) -> &mut Self {
        self.0.push_flag(CC_KEY);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn login_shell(&mut self) -> &mut Self {
        self.0.push_flag(l_KEY);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn force_utf8(&mut self) -> &mut Self {
        self.0.push_flag(u_KEY);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn verbose_logging(&mut self) -> &mut Self {
        self.0.push_flag(v_KEY);
        self
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn shell_cmd<S: Into<Cow<'a, str>>>(&mut self, shell_cmd: S) -> &mut Self {
        self.0.push_option(c_KEY, shell_cmd);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn file<S: Into<Cow<'a, str>>>(&mut self, file: S) -> &mut Self {
        self.0.push_option(f_KEY, file);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn socket_name<S: Into<Cow<'a, str>>>(&mut self, socket_name: S) -> &mut Self {
        self.0.push_option(L_KEY, socket_name);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn socket_path<S: Into<Cow<'a, str>>>(&mut self, socket_path: S) -> &mut Self {
        self.0.push_option(S_KEY, socket_path);
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

    pub fn exec(&self) -> TmuxOutput {
        self.0.exec()
    }

    pub fn build(self) -> Self {
        self
    }
}

impl<'a> From<Tmux<'a>> for TmuxCommand<'a> {
    fn from(item: Tmux<'a>) -> Self {
        item.0
    }
}

impl<'a> From<&Tmux<'a>> for TmuxCommand<'a> {
    fn from(item: &Tmux<'a>) -> Self {
        item.0.clone()
    }
}

//impl<'a> MyCommand<'a> for Tmux<'a> {
//fn set_bin<S: Into<Cow<'a, str>>>(&mut self, bin: S) -> &mut Self {
//self.0.bin = Some(bin.into());
//self
//}

//fn get_bin(&self) -> Option<&Cow<'a, str>> {
//self.0.bin.as_ref()
//}
//}
