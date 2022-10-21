mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// use inside module
mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant2() {
        hosting::add_to_waitlist();
    }
}

// the same name in scope
use std::fmt;
use std::io;

fn function1() -> fmt::Result {}

fn function2() -> io::Result<()> {
    // snip--
}

// Providing New Names with the as Keyword
use std::io::Result as IoResult;

fn function3() -> IoResult<()> {
    //--snip--
}
