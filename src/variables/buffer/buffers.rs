use crate::{Buffer, Error};
use std::ops::Index;
use std::str::FromStr;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Buffers(pub Vec<Buffer>);

impl IntoIterator for Buffers {
    type Item = Buffer;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Index<usize> for Buffers {
    type Output = Buffer;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl FromStr for Buffers {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let mut buffers = Buffers::new();
        for line in s.lines() {
            buffers.push(Buffer::from_str(line)?);
        }
        Ok(buffers)
    }
}

impl Buffers {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, buffer: Buffer) {
        self.0.push(buffer);
    }

    //let sessions_str = String::from_utf8_lossy(&output.0.stdout.as_slice());
    //Buffers::from_str(&sessions_str, bitflags)
    //}
}
