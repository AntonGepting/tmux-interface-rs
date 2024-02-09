use crate::Error;
use crate::FormatsOutput;
use std::str::FromStr;

// XXX: 1.9 processed
#[derive(Default, PartialEq, Clone, Debug)]
pub struct Buffer {
    /// buffer_created - Time buffer created
    #[cfg(feature = "tmux_2_6")]
    pub created: Option<u128>,
    /// buffer_name - Name of buffer
    #[cfg(feature = "tmux_2_3")]
    pub name: Option<String>,
    /// buffer_sample - First 50 characters from the specified buffer
    #[cfg(feature = "tmux_1_7")]
    pub sample: Option<String>,
    /// buffer_size - Size of the specified buffer in bytes
    #[cfg(feature = "tmux_1_7")]
    pub size: Option<usize>,
}

impl FromStr for Buffer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let mut buffer = Buffer::new();
        let mut format = FormatsOutput::new();
        format.separator(':');

        #[cfg(feature = "tmux_2_6")]
        format.buffer_created(&mut buffer.created);
        #[cfg(feature = "tmux_2_3")]
        format.buffer_name(&mut buffer.name);
        #[cfg(feature = "tmux_1_7")]
        format.buffer_sample(&mut buffer.sample);
        #[cfg(feature = "tmux_1_7")]
        format.buffer_size(&mut buffer.size);

        FormatsOutput::from_string_ext(s, &mut format);
        Ok(buffer)
    }
}

impl Buffer {
    pub fn new() -> Self {
        Default::default()
    }

    // XXX: wrapper with format generating and result parsing using callback
}
