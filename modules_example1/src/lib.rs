mod front_of_house; // Using ; instead of {} tells rust to load the contents of module from another file

pub use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast { // Because this struct has private field seasonal_fruit, we need to provide a function to create one
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn new(toast : &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
        pub fn getSecret(&self) -> &String {
            &self.seasonal_fruit
        }
    }

    pub enum Appetizer { // On enums the pub effects all
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();  // Absolute path
    front_of_house::hosting::add_to_waitlist();         // Relative path

    let mut meal = back_of_house::Breakfast::new("Rye bread");
    meal.toast = String::from("Potato bread");
    let order1 = back_of_house::Appetizer::Salad;
}

// use crate::front_of_house::hosting;
// or use self::front_of_house::hosting::add_to_waitlist;

pub fn eaty_eaty() {
    hosting::add_to_waitlist();
}

use std::fmt::Result;
pub use std::io::Result as IoResult;    // pub use re-exports std::io:Result as IoResult

// Another way to using the two lines above
// use std::{fmt::Result, io::Result as IoResult};

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}