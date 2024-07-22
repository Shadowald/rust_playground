// how to include multiple things from the same lib without using a line for each one
use rand::{Rng, CryptoRng, RngCore};
// to include everything, you can * as a glob to include all public items
use std::io::*;

// all modules and functions are private unless given the public(pub) modifier
mod front_of_house;

mod back_of_house{
    fn fix_incorrect_order() {
        cook_order();
        // super refrences the parent module which is crate in this case
        super::serve_order();
    }

    fn cook_order() {}

    pub  struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Can use the use keyword to like using::namespace to reduce having to write out the entire path
//use crate::front_of_house::hosting;     //Absoulte path
use self::front_of_house::hosting;      //Relative path

// Need to specify the path to access a function in a module(mod)
pub fn eat_at_restraunt() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();      //without use key word
    hosting::add_to_waitlist();                             //with use key word

    // Relative path - inside current module
    front_of_house::hosting::add_to_waitlist();             //Without use key word
    hosting::add_to_waitlist();                             //with use key word

    let mut meal = back_of_house::Breakfast::summer("rye");

    meal.toast = String::from("wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}
