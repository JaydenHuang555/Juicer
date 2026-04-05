use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct TeamNumber {
    pub number: u32,
}

impl TeamNumber {
    pub fn digits(&self) -> u32 {
        if self.number == 0 {
            return 1;
        }

        self.number.ilog10() + 1
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn v4addr(&self, hostid: u8) -> Ipv4Addr {
        let te = self.number / 100;
        let am = self.number % 100;
        return Ipv4Addr::new(10, te as u8, am as u8, hostid);
    }
}

impl From<u32> for TeamNumber {
    fn from(value: u32) -> Self {
        Self { number: value }
    }
}
