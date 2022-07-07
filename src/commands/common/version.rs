use crate::Error;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
enum VersionState {
    ProgName,
    Prefix,
    Major,
    Minor,
    Suffix,
    //Error
}

#[derive(Default, Clone, Debug)]
pub struct Version {
    pub prog_name: String,
    pub prefix: String,
    pub major: usize,
    pub minor: usize,
    pub suffix: String,
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(version_str: &str) -> Result<Self, Self::Err> {
        let mut version = Version::new();
        let chars = version_str.chars();
        let mut state = VersionState::ProgName;
        let mut buff = String::new();

        // fsm for parsing
        // XXX: optimization?
        for c in chars {
            match (c, &state) {
                // end of program name part
                (' ', VersionState::ProgName) => {
                    state = VersionState::Major;
                }
                ('a'..='z', VersionState::Major) => {
                    state = VersionState::Prefix;
                    version.prefix.push(c);
                }
                // end of major part
                ('.', VersionState::Major) => {
                    state = VersionState::Minor;
                    version.major = buff.parse()?;
                    buff = String::new();
                }
                // end of prefix part, begin major
                ('-', VersionState::Prefix) => {
                    state = VersionState::Major;
                }
                // end of minor part & EOL
                ('\n', VersionState::Minor) => {
                    version.minor = buff.parse()?;
                    break;
                }
                // end of minor part, begin alpha, beta
                ('a'..='z', VersionState::Minor) => {
                    version.minor = buff.parse()?;
                    state = VersionState::Suffix;
                    version.suffix.push(c);
                }
                // end of minor part, begin -rc
                ('-', VersionState::Minor) => {
                    version.minor = buff.parse()?;
                    state = VersionState::Suffix;
                }
                // end of suffix part & EOL
                ('\n', VersionState::Suffix) => {
                    break;
                }
                ('a'..='z', VersionState::ProgName) => {
                    version.prog_name.push(c);
                }
                ('a'..='z', VersionState::Prefix) => {
                    version.prefix.push(c);
                }
                ('a'..='z', VersionState::Suffix) => {
                    version.suffix.push(c);
                }
                (_, _) => {
                    buff.push(c);
                    //state = TmuxVersionState::Error;
                }
            }
        }
        Ok(version)
    }
}

impl Version {
    pub fn new() -> Self {
        Default::default()
    }
}
