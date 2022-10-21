mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// Using Nested Paths to Clean Up Large use Lists

use std::{cmp::Ordering, io};

// instead of
// use std::io;
// use std::io::Write;
// we can do
// use std::io::{self, Write};

// The Glob Operator
use std::collections::*;
