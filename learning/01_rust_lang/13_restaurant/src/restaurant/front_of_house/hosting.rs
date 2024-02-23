use rand::Rng;

pub fn add_to_waitlist(person: &str) {
    println!("Added to waitlist a {}", person);
}

pub fn seat_at_table() -> u32 {
    rand::thread_rng().gen_range(1..101)
}
