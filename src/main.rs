use rand::Rng;
use rand::distributions::Alphanumeric;
use std::char;

fn generate_random_key(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| char::from(rng.sample(Alphanumeric)))
        .collect::<String>()
}

fn main() {
    println!("\nWelcome to the Random Keys Generator in Rust!\n");
    let key_length = 32;
    println!("Generated key: {}\n", generate_random_key(key_length));
}