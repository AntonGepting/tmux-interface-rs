use crate::Error;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "tmux_3_1")]
pub mod pane_options;
#[cfg(feature = "tmux_1_2")]
pub mod server_options;
#[cfg(feature = "tmux_1_0")]
pub mod session_options;
#[cfg(feature = "tmux_1_2")]
pub mod window_options;

#[cfg(feature = "tmux_3_1")]
pub mod pane_options_tests;
#[cfg(feature = "tmux_1_2")]
pub mod server_options_tests;
#[cfg(feature = "tmux_1_0")]
pub mod session_options_tests;
#[cfg(feature = "tmux_1_2")]
pub mod window_options_tests;

#[cfg(feature = "tmux_3_1")]
use crate::options::pane_options::PaneOptions;
#[cfg(feature = "tmux_1_2")]
use crate::options::server_options::ServerOptions;
#[cfg(feature = "tmux_1_0")]
use crate::options::session_options::SessionOptions;
#[cfg(feature = "tmux_1_2")]
use crate::options::window_options::WindowOptions;

#[cfg(feature = "tmux_1_0")]
pub struct Options {
    pub server_options: ServerOptions,
    pub session_options: SessionOptions,
    pub window_options: WindowOptions,
    #[cfg(feature = "tmux_3_1")]
    pub pane_options: PaneOptions,
}

#[derive(PartialEq, Clone, Debug)]
pub enum StatusKeys {
    Vi,
    Emacs,
}

impl FromStr for StatusKeys {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "vi" => Ok(Self::Vi),
            "emacs" => Ok(Self::Emacs),
            _ => Err(Error::ParseStatusKeys),
        }
    }
}

impl fmt::Display for StatusKeys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Vi => write!(f, "vi"),
            Self::Emacs => write!(f, "emacs"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Switch {
    On,
    Off,
}

impl FromStr for Switch {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err(Error::ParseSwitch),
        }
    }
}

impl fmt::Display for Switch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
        }
    }
}

// fn checks if vec already exists, if not creates it, and inserts an item at given index
#[cfg(feature = "tmux_1_0")]
fn create_insert_vec(
    v: Option<&mut Vec<String>>,
    i: Option<usize>,
    s: &str,
) -> Option<Vec<String>> {
    if let Some(v) = v {
        if let Some(i) = i {
            v.insert(i, s.to_string());
            return Some(v.to_vec());
        };
    } else {
        let mut v = Vec::new();
        if let Some(i) = i {
            v.insert(i, s.to_string());
            return Some(v);
        };
    };
    None
}
