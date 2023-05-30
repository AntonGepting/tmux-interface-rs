use crate::Error;
use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

const SEPARATOR: &str = ":";

const NUMBER_256: &str = "256";
const CLIPBOARD: &str = "clipboard";
const CCOLOUR: &str = "ccolour";
const CSTYLE: &str = "cstyle";
const EXTKEYS: &str = "extkeys";
const FOCUS: &str = "focus";
const MARGINS: &str = "margins";
const MOUSE: &str = "mouse";
const OVERLINE: &str = "overline";
const RECTFILL: &str = "rectfill";
const RGB: &str = "RGB";
const STRIKETHROUGH: &str = "strikethrough";
const SYNC: &str = "sync";
const TITLE: &str = "title";
const USSTYLE: &str = "usstyle";

#[derive(Default, PartialEq, Clone, Debug)]
pub struct TerminalFeatures<'a> {
    /// terminal type pattern (matched using fnmatch(3))
    pub terminal_type: Cow<'a, str>,
    /// Supports 256 colours with the SGR escape sequences.
    pub colours256: bool,
    /// Allows setting the system clipboard.
    pub clipboard: bool,
    /// Allows setting the cursor colour.
    pub ccolour: bool,
    /// Allows setting the cursor style.
    pub cstyle: bool,
    /// Supports extended keys.
    pub extkeys: bool,
    /// Supports focus reporting.
    pub focus: bool,
    /// Supports DECSLRM margins.
    pub margins: bool,
    /// Supports xterm(1) mouse sequences.
    pub mouse: bool,
    /// Supports the overline SGR attribute.
    pub overline: bool,
    /// Supports the DECFRA rectangle fill escape sequence.
    pub rectfill: bool,
    /// Supports RGB colour with the SGR escape sequences.
    pub rgb: bool,
    /// Supports the strikethrough SGR escape sequence.
    pub strikethrough: bool,
    /// Supports synchronized updates.
    pub sync: bool,
    /// Supports xterm(1) title setting.
    pub title: bool,
    /// Allows underscore style and colour to be set.
    pub usstyle: bool,
}

impl<'a> fmt::Display for TerminalFeatures<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        if self.colours256 {
            v.push(NUMBER_256);
        }
        if self.clipboard {
            v.push(CLIPBOARD);
        }
        if self.clipboard {
            v.push(CLIPBOARD);
        }
        if self.ccolour {
            v.push(CCOLOUR);
        }
        if self.cstyle {
            v.push(CSTYLE);
        }
        if self.extkeys {
            v.push(EXTKEYS);
        }
        if self.focus {
            v.push(FOCUS);
        }
        if self.margins {
            v.push(MARGINS);
        }
        if self.mouse {
            v.push(MOUSE);
        }
        if self.overline {
            v.push(OVERLINE);
        }
        if self.rectfill {
            v.push(RECTFILL);
        }
        if self.rgb {
            v.push(RGB);
        }
        if self.strikethrough {
            v.push(STRIKETHROUGH);
        }
        if self.sync {
            v.push(SYNC);
        }
        if self.title {
            v.push(TITLE);
        }
        if self.usstyle {
            v.push(USSTYLE);
        }

        let s = v.join(":");
        write!(f, "{}:{}", self.terminal_type, s)
    }
}

impl<'a> TerminalFeatures<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn terminal_type<S: Into<Cow<'a, str>>>(&mut self, terminal_type: S) -> &mut Self {
        self.terminal_type = terminal_type.into();
        self
    }

    pub fn _256(&mut self, flag: bool) -> &mut Self {
        self.colours256 = flag;
        self
    }

    pub fn clipboard(&mut self, flag: bool) -> &mut Self {
        self.clipboard = flag;
        self
    }

    pub fn ccolour(&mut self, flag: bool) -> &mut Self {
        self.ccolour = flag;
        self
    }

    pub fn cstyle(&mut self, flag: bool) -> &mut Self {
        self.cstyle = flag;
        self
    }

    pub fn extkeys(&mut self, flag: bool) -> &mut Self {
        self.extkeys = flag;
        self
    }

    pub fn focus(&mut self, flag: bool) -> &mut Self {
        self.focus = flag;
        self
    }

    pub fn margins(&mut self, flag: bool) -> &mut Self {
        self.margins = flag;
        self
    }

    pub fn mouse(&mut self, flag: bool) -> &mut Self {
        self.mouse = flag;
        self
    }

    pub fn overline(&mut self, flag: bool) -> &mut Self {
        self.overline = flag;
        self
    }

    pub fn rectfill(&mut self, flag: bool) -> &mut Self {
        self.rectfill = flag;
        self
    }

    pub fn rgb(&mut self, flag: bool) -> &mut Self {
        self.rgb = flag;
        self
    }

    pub fn strikethrough(&mut self, flag: bool) -> &mut Self {
        self.strikethrough = flag;
        self
    }

    pub fn sync(&mut self, flag: bool) -> &mut Self {
        self.sync = flag;
        self
    }

    pub fn title(&mut self, flag: bool) -> &mut Self {
        self.title = flag;
        self
    }

    pub fn usstyle(&mut self, flag: bool) -> &mut Self {
        self.usstyle = flag;
        self
    }
}

//
// ```
// xterm*:clipboard:ccolour:cstyle:focus:title
// screen*:title
// ```
impl<'a> FromStr for TerminalFeatures<'a> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tf = TerminalFeatures::default();
        let v: Vec<&str> = s.trim().splitn(2, SEPARATOR).collect();

        tf.terminal_type = v[0].to_owned().into();

        let v: Vec<&str> = v[1].split(SEPARATOR).collect();

        for feature in v {
            match feature {
                NUMBER_256 => tf.colours256 = true,
                CLIPBOARD => tf.clipboard = true,
                CCOLOUR => tf.ccolour = true,
                CSTYLE => tf.cstyle = true,
                EXTKEYS => tf.extkeys = true,
                FOCUS => tf.focus = true,
                MARGINS => tf.margins = true,
                MOUSE => tf.mouse = true,
                OVERLINE => tf.overline = true,
                RECTFILL => tf.rectfill = true,
                RGB => tf.rgb = true,
                STRIKETHROUGH => tf.strikethrough = true,
                SYNC => tf.sync = true,
                TITLE => tf.title = true,
                USSTYLE => tf.usstyle = true,
                _ => return Err(Error::ParseTerminalFeatures),
            }
        }

        Ok(tf)
    }
}
