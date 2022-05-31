use crate::restaurant::eat_at_restaurant;
use crate::restaurant::customer::eat_at_restaurant as customer_eat_at_restaurant;
use crate::restaurant::hosting::seat_at_table;

mod restaurant;

fn main() {
    eat_at_restaurant();
    customer_eat_at_restaurant();
    let table = seat_at_table();
    println!("Seat at table {}", table);
}
