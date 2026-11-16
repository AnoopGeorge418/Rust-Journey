use std::collections::HashMap as Dictionary; // alias name

// nested module use // can also write this in 1 line but with same syntax
use std::{
    cmp::Ordering,
    io::stdin,
};

// bringing all paths to scope using glob operator
use std::io::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn server_order() {}

        pub fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let mut map = Dictionary::new();
    map.insert(1, 2);
}
