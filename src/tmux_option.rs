use super::error::Error;
use super::ShowOptions;
use super::TmuxInterface;

pub struct TmuxOption;

impl TmuxOption {
    pub fn get_int(option_name: &str) -> Result<usize, Error> {
        let mut tmux = TmuxInterface::new();
        let show_options = ShowOptions {
            global: Some(true),
            #[cfg(feature = "tmux_1_8")]
            value: Some(true),
            #[cfg(feature = "tmux_1_7")]
            option: Some(option_name),
            ..Default::default()
        };
        let value = tmux.show_options(Some(&show_options))?;
        let value_parts: Vec<&str> = value.split('\n').collect();
        let int = value_parts[0][0..].parse::<usize>()?;
        Ok(int)
    }
}
