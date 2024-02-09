use crate::{Client, Error};
use std::ops::Index;
use std::str::FromStr;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Clients(pub Vec<Client>);

impl IntoIterator for Clients {
    type Item = Client;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Index<usize> for Clients {
    type Output = Client;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl FromStr for Clients {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let mut clients = Clients::new();
        for line in s.lines() {
            clients.push(Client::from_str(line)?);
        }
        Ok(clients)
    }
}

impl Clients {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, client: Client) {
        self.0.push(client);
    }

    //let sessions_str = String::from_utf8_lossy(&output.0.stdout.as_slice());
    //Clients::from_str(&sessions_str, bitflags)
    //}
}
