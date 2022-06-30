use super::Style;
use std::fmt;

const SPACE_SEPRATOR: &str = " ";
const COMMA_SEPRATOR: &str = ",";

//
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct StyleList<'a> {
    pub styles: Option<Vec<Style>>,
    pub separator: Option<&'a str>,
}

impl<'a> fmt::Display for StyleList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        if let Some(styles) = &self.styles {
            for style in styles {
                v.push(style.to_string());
            }
        }
        let s = v.join(self.separator.unwrap_or(COMMA_SEPRATOR));
        write!(f, "{}", s)
    }
}

impl<'a> StyleList<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, style: Style) -> &mut Self {
        self.styles.get_or_insert(Vec::new()).push(style);
        self
    }

    pub fn space_separator(&mut self) -> &mut Self {
        self.separator = Some(SPACE_SEPRATOR);
        self
    }

    pub fn comma_separator(&mut self) -> &mut Self {
        self.separator = Some(COMMA_SEPRATOR);
        self
    }
}
