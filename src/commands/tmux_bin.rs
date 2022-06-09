use crate::commands::constants::*;
use std::borrow::Cow;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TmuxBin<'a> {
    /// Tmux environment variables
    pub env: Option<Vec<(&'a str, &'a str)>>,
    /// Tmux executable name, (part I)
    pub bin: Option<Cow<'a, str>>,
    /// Tmux executable arguments (part II)
    pub args: Option<Vec<&'a str>>,
}

const TMUX_BIN_SEPARATOR: &str = " ";

// XXX: implement env support
impl<'a> fmt::Display for TmuxBin<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();

        // push environment variables
        //if let Some(env) = &self.env {
        //args.iter().for_each(|s| v.push(s.as_ref()));
        //}

        // push executable name
        v.push(self.bin.as_ref());

        // push args
        if let Some(args) = &self.args {
            args.iter().for_each(|s| v.push(s.as_ref()));
        }

        let output = v.join(TMUX_BIN_SEPARATOR);
        write!(f, "{}", output)
    }
}

impl<'a> Default for TmuxBin<'a> {
    fn default() -> Self {
        TmuxBin {
            env: None,
            bin: None,
            args: None,
        }
    }
}

impl<'a> TmuxBin<'a> {
    pub fn new() -> Self {
        TmuxBin::default()
    }

    /// Returns mutable reference to tmux executable name `Cow<'a, str>`
    ///
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use tmux_interface::commands::tmux_bin::TmuxBin;
    ///
    /// let mut tmux = TmuxBin::default();
    /// let bin = tmux.bin();
    /// assert_eq!(bin, &Cow::Borrowed("tmux"));
    /// ```
    pub fn bin(&self) -> &Cow<'a, str> {
        &self.bin
    }

    /// Returns mutable reference to tmux executable name `Cow<'a, str>`
    ///
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use tmux_interface::commands::tmux_bin::TmuxBin;
    ///
    /// let mut tmux = TmuxBin::default();
    /// *tmux.bin_mut() = Cow::Borrowed("/usr/bin/tmux");
    /// assert_eq!(tmux.bin, Cow::Borrowed("/usr/bin/tmux"));
    /// ```
    /// or
    /// ```
    /// use std::borrow::Cow;
    /// use tmux_interface::commands::tmux_bin::TmuxBin;
    ///
    /// let mut tmux = TmuxBin::default();
    /// *tmux.bin_mut() = "/usr/bin/tmux".into();
    /// assert_eq!(tmux.bin, Cow::Borrowed("/usr/bin/tmux"));
    /// ```
    pub fn bin_mut(&mut self) -> &mut Cow<'a, str> {
        &mut self.bin
    }

    pub fn args(&self) -> &Option<Vec<Cow<'a, str>>> {
        &self.args
    }

    pub fn args_mut(&mut self) -> &mut Option<Vec<Cow<'a, str>>> {
        &mut self.args
    }

    pub fn verbose(&mut self) -> &mut Self {
        self.args = Some(vec![Cow::Borrowed("-v")]);
        self
    }
}
