use super::error::Error;
use super::ShowOptions;
use super::TmuxInterface;
use crate::TargetPane;

pub struct TmuxOption;

impl TmuxOption {
    pub fn get_int(option_name: &str) -> Result<usize, Error> {
        let mut tmux = TmuxInterface::new();
        let show_options = ShowOptions::<TargetPane> {
            global_options: Some(true),
            option_value: Some(true),
            option: Some(option_name),
            ..Default::default()
        };
        let value = tmux.show_options(Some(&show_options))?;
        let value_parts: Vec<&str> = value.split('\n').collect();
        let int = value_parts[0][0..].parse::<usize>()?;
        Ok(int)
    }
}
