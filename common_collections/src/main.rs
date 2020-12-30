mod explore_vector;
mod explore_string;
mod explore_hash_map;

fn main() {
    println!("Running explore_vector::run()");
    explore_vector::run();

    println!("Running explore_string::run()");
    explore_string::run();

    println!("Running explore_hash_map::run()");
    explore_hash_map::run();
}
