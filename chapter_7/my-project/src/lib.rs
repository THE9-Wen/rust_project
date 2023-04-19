mod front_of_house;

// use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting;

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}

pub fn eat_at_restaurant() {
    // absolute
    crate::front_of_house::hosting::add_to_waitlist();

    // relative
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    // meal.season_fruit = String::from("Watermelon");

    let soup = back_of_house::Appetizer::Soup;
    let salad = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        season_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("peaches"),
            }
        }
    }
}
