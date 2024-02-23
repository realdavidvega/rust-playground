pub mod front_of_house;
pub mod back_of_house;

// We bring the module into scope
// We also make it public for external code
pub use front_of_house::hosting;

pub mod customer {
    pub fn eat_at_restaurant() {
        // Error: not in this scope
        //hosting::add_to_waitlist();

        // This works :)
        super::hosting::add_to_waitlist("customer");
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    //front_of_house::hosting::add_to_waitlist();

    // With use front_of_house::hosting;
    hosting::add_to_waitlist("special customer");

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    println!("Special customer: I'd like {} toast please", meal.toast);
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("Special customer: Sorry, I've changed my mind, I'd prefer {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}