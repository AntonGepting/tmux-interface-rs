use super::ShowOptions;
use super::TmuxInterface;
use super::tmux_interface_error::TmuxInterfaceError;
use regex::Regex;
use std::borrow::Cow;


pub struct TmuxOption;


impl TmuxOption {


    const GET_INT_OPTION_REGEX: &'static str = r"^(\d+)\n$";


    pub fn get_int(option_name: &str) -> Result<usize, TmuxInterfaceError>{
        let tmux = TmuxInterface::new();
        let show_options = ShowOptions {
            global_options: Some(true),
            option_value: Some(true),
            option: Some(Cow::Borrowed(option_name)),
            ..Default::default()
        };
        let value = tmux.show_options(&show_options)?;
        let regex = Regex::new(TmuxOption::GET_INT_OPTION_REGEX)?;
        if let Some(caps) = regex.captures(&value) {
            let int = caps[1].parse::<usize>()?;
            return Ok(int);
        } else {
            return Err(TmuxInterfaceError::new("regex parse error"));
        }
    }


}
