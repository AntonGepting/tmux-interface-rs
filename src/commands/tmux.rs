use crate::commands::constants::*;
use crate::TmuxCommand;

#[derive(Default, Debug, Clone)]
pub struct Tmux<'a>(TmuxCommand<'a>);

//impl Default for Tmux {
//fn default() -> Self {
//Self(TmuxCommand {
//cmd: Some(NewSession::NEW_SESSION.to_string()),
//..Default::default()
//})
//}
//}

// XXX: using environment vars
impl<'a> Tmux<'a> {
    pub const TMUX: &'static str = "tmux";

    pub fn new() -> Self {
        Default::default()
    }

    //fn bin<S: Into<Cow<'a, str>>>(mut self, bin: S) -> Self {
    //self.bin = Some(bin.into());
    //self
    //}

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
    pub fn shell_cmd<S: Into<String>>(&mut self, shell_cmd: S) -> &mut Self {
        self.0.push_option(c_KEY, shell_cmd);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn file<S: Into<String>>(&mut self, file: S) -> &mut Self {
        self.0.push_option(f_KEY, file);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn socket_name<S: Into<String>>(&mut self, socket_name: S) -> &mut Self {
        self.0.push_option(L_KEY, socket_name);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn socket_path<S: Into<String>>(&mut self, socket_path: S) -> &mut Self {
        self.0.push_option(S_KEY, socket_path);
        self
    }

    //fn output(&self) -> TmuxOutput {
    //self.output()
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
