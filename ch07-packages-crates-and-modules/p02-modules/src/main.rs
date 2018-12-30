#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fmt::Result;
use std::io::Result as IoResult;

use std::io::{self, Write};

trait Junk {
    fn function1() -> Result;
    fn function2() -> IoResult<()>;
}

mod sound {
    pub mod instrument {
        pub fn clarinet() {
            super::breathe_in();
        }
    }

    fn breathe_in() {
        println!("Breath in !");
    }
}

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();
    // Relative path
    sound::instrument::clarinet();

    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();
}
