use crate::Error;
use std::str::FromStr;

const WINDOW_FLAG_DEFAULT: usize = 0b0000_0000;
const WINDOW_FLAG_CURRENT: usize = 0b0000_0001;
const WINDOW_FLAG_LAST: usize = 0b0000_0010;
const WINDOW_FLAG_ACTIVITY: usize = 0b0000_0100;
const WINDOW_FLAG_BELL: usize = 0b0000_1000;
const WINDOW_FLAG_SILENCED: usize = 0b0001_0000;
const WINDOW_FLAG_MARKED: usize = 0b0010_0000;
const WINDOW_FLAG_ZOOMED: usize = 0b0100_0000;

#[derive(Clone, PartialEq, Debug)]
pub struct WindowFlag(usize);

impl Default for WindowFlag {
    fn default() -> Self {
        WindowFlag(WINDOW_FLAG_DEFAULT)
    }
}

impl FromStr for WindowFlag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut wf = WindowFlag(WINDOW_FLAG_DEFAULT);
        let chrs = s.chars();
        for c in chrs {
            match c {
                '*' => wf.0 += WINDOW_FLAG_CURRENT,
                '-' => wf.0 += WINDOW_FLAG_LAST,
                '#' => wf.0 += WINDOW_FLAG_ACTIVITY,
                '!' => wf.0 += WINDOW_FLAG_BELL,
                '~' => wf.0 += WINDOW_FLAG_SILENCED,
                'M' => wf.0 += WINDOW_FLAG_MARKED,
                'Z' => wf.0 += WINDOW_FLAG_ZOOMED,
                // XXX: Error description
                _ => return Err(Error::ParseWindowFlag),
            }
        }
        Ok(wf)
    }
}
