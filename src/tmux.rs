use crate::tmux_command::TmuxCommand;
use crate::tmux_interface::*;
use crate::tmux_output::TmuxOutput;
use std::borrow::Cow;

#[derive(Default, Debug, Clone)]
pub struct Tmux<'a>(TmuxCommand<'a>);

// XXX: using environment vars
impl<'a> Tmux<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn version(&mut self) -> &mut Self {
        self.0.insert_flag(V_KEY);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn colours256(&mut self) -> &mut Self {
        self.0.insert_flag(_2_KEY);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn control_mode(&mut self) -> &mut Self {
        self.0.insert_flag(C_KEY);
        self
    }

    pub fn disable_echo(&mut self) -> &mut Self {
        self.0.insert_flag(CC_KEY);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn login_shell(&mut self) -> &mut Self {
        self.0.insert_flag(l_KEY);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn force_utf8(&mut self) -> &mut Self {
        self.0.insert_flag(u_KEY);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn verbose_logging(&mut self) -> &mut Self {
        self.0.insert_flag(v_KEY);
        self
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn shell_cmd<S: Into<Cow<'a, str>>>(&mut self, shell_cmd: S) -> &mut Self {
        self.0.insert_option(c_KEY, shell_cmd);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn file<S: Into<Cow<'a, str>>>(&mut self, file: S) -> &mut Self {
        self.0.insert_option(f_KEY, file);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn socket_name<S: Into<Cow<'a, str>>>(&mut self, socket_name: S) -> &mut Self {
        self.0.insert_option(L_KEY, socket_name);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn socket_path<S: Into<Cow<'a, str>>>(&mut self, socket_path: S) -> &mut Self {
        self.0.insert_option(S_KEY, socket_path);
        self
    }

    // XXX: ???
    //pub fn cmd(self) -> TmuxCommand<'a> {
    //self.0
    //}

    pub fn exec(&mut self) -> TmuxOutput {
        self.0.exec()
    }
}

impl<'a> From<Tmux<'a>> for TmuxCommand<'a> {
    fn from(item: Tmux<'a>) -> Self {
        item.0
    }
}
