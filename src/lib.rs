mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {
            super::hosting::add_to_waitlist();
        }
    }
}
mod back_of_house {
    fn fix_incorrect_orfer() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }
    fn cook_order() {}
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Apple"),
            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
use crate::front_of_house::*;
pub use std::{collections::HashMap, fmt::Result, io::Result as IoResult};
//pub use std::io::Result as IoResult;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("white");
    meal.toast = String::from("black");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let hash_map:HashMap<&str, i32> = HashMap::new();
    // let a = Result::Ok(5);
    // let b = IoResult::Ok((5));
}

//End 164
