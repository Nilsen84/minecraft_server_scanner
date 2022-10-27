use std::net::Ipv4Addr;

use rand::{RngCore, thread_rng};

pub struct IpIterator {
    active: bool,
    start: u32,
    cur: u32
}

impl IpIterator {
    pub fn new() -> Self {
        let start = thread_rng().next_u32().min(1);

        Self {
            active: true,
            start,
            cur: start
        }
    }
}

impl Iterator for IpIterator {
    type Item = Ipv4Addr;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.active {
            return None;
        }

        //https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Galois_LFSRs

        let lsb = self.cur & 1;
        self.cur >>= 1;
        if lsb == 1 {
            self.cur ^= 0xccf2e57e;
        }

        if self.cur == self.start {
            self.active = false;
        }

        Some(
            Ipv4Addr::from(self.cur)
        )
    }
}