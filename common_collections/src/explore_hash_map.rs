use std::collections::HashMap;

pub fn run() {
    /*
        HashMap<K, V> stores a mapping of keys
        of type K to values of type V
    */

    // creating a new HashMap and inserting values
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for score in scores {
        println!("{:#?}", score);
    }

    println!();

    // creating a HashMap using iterators and the collect method on a vector of tuples
    let teams = vec![
        String::from("Blue"),
        String::from("Yellow")
    ];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_,_> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    for score in &scores {
        println!("{:#?}", score);
    }

    println!();

    // ownership

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name and field_value can no longer be used

    // accessing values
    let color = map.get("Favorite color");

    match color {
        Some(c) => println!("Favorite color is: {}", c),
        None => println!("'Favorite color' was not found.")
    }

    println!();

    // iterating through values

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!();

    // updating a value
    map.insert(String::from("Favorite color"), String::from("Green"));
    let color = map.get("Favorite color");
    
    if let Some(c) = color {
        println!("Favorite color updated: {}", c);
    } else {
        println!("'Favorite color' was not found.");
    }

    println!();

    // only insert if the key has no value
    map.entry(String::from("Favorite color")).or_insert(String::from("Red"));
    map.entry(String::from("Favorite band")).or_insert(String::from("Foo Fighters"));

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    println!();

    // updating value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
    println!();
}