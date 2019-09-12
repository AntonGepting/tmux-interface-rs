use super::ShowOptions;
use super::TmuxInterface;
use super::error::Error;


pub struct TmuxOption;


impl TmuxOption {


    pub fn get_int(option_name: &str) -> Result<usize, Error>{
        let tmux = TmuxInterface::new();
        let show_options = ShowOptions {
            global_options: Some(true),
            option_value: Some(true),
            option: Some(option_name),
            ..Default::default()
        };
        let value = tmux.show_options(&show_options)?;
        let value_parts: Vec<&str> = value.split('\n').collect();
        let int = value_parts[0][0..].parse::<usize>()?;
        Ok(int)
    }


}
