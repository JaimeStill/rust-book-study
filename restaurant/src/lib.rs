/*
    Using a semi-colon here rather than a block
    tells Rust to load the contents of the module
    from another file with the same name as
    the module.
*/
mod front_of_house;

pub mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad
    }

    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches")
            }
        }
    }

    fn _cook_order() { }

    fn _fix_incorrect_order() {
        _cook_order();

        // relative path to serve_order() defined up one level
        super::_serve_order();
    }
}

fn _serve_order() { }

// bring the hosting namespace into scope
use crate::front_of_house::hosting;

/*
    bring the back_of_house namespace into scope, 
    as well as Breakfast and Appetizer, 
    using nested path and setting an alias for
    Breakfast as Bfast

    NOTE: to also include the specified namespace, use self:
    use crate::module::sub_module::{self, ...}
*/
use crate::back_of_house::{ Breakfast as Bfast, Appetizer };

pub fn eat_at_restaurant() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    // front_of_house::hosting::add_to_waitlist();

    // call add_to_waitlist() from the hosting namespace
    hosting::add_to_waitlist();

    let mut meal = Bfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let _order1 = Appetizer::Soup;
    let _order2 = Appetizer::Salad;
}