use crate::Error;
use std::str::FromStr;


#[derive(PartialEq, Debug, Clone)]
enum VersionState {
    ProgName,
    Major,
    Minor,
    Suffix,
    //Error
}


#[derive(Default, Clone, Debug)]
pub struct Version {
    pub prog_name: String,
    pub major: usize,
    pub minor: usize,
    pub suffix: String
}


impl FromStr for Version {
    type Err = Error;

    fn from_str(version_str: &str) -> Result<Self, Self::Err> {
        let mut version = Version::new();
        let mut chars = version_str.chars();
        let mut state = VersionState::ProgName;
        let mut buff = String::new();

        // fsm for parsing
        // XXX: optimization?
        loop {
            if let Some(c) = chars.next() {
                match (c, &state) {
                    // end of program name part
                    (' ', VersionState::ProgName) => {
                        state = VersionState::Major;
                    },
                    // end of major part
                    ('.', VersionState::Major) => {
                        state = VersionState::Minor;
                        version.major = buff.parse()?;
                        buff = String::new();
                    },
                    // end of minor part & EOL
                    ('\n', VersionState::Minor) => {
                        version.minor = buff.parse()?;
                        break;
                    },
                    // end of minor part
                    ('a'..='z', VersionState::Minor) => {
                        version.minor = buff.parse()?;
                        state = VersionState::Suffix;
                        version.suffix.push(c);
                    },
                    // end of suffix part & EOL
                    ('\n', VersionState::Suffix) => {
                        break;
                    },
                    ('a'..='z', VersionState::ProgName) => {
                        version.prog_name.push(c);
                    },
                    ('a'..='z', VersionState::Suffix) => {
                        version.suffix.push(c);
                    },
                    (_, _) => {
                        buff.push(c);
                        //state = TmuxVersionState::Error;
                    }
                }
            } else {
                break;
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
