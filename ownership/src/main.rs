use std::io;

fn main() {
    let s = String::from("Hello!");
    print_string(s); // this instance of s can no longer be used after this point

    let s = print_and_return(create_string());
    println!("{} is still in scope", s);

    // argument passed as a reference to prevent passing ownership
    println!("Length of {}: {}", s, calculate_length(&s));

    // mutable reference modified by a function
    let mut s = String::from("hello");
    change_string(&mut s);
    println!("Modified s: {}", s);

    let s = String::from("hello world");
    let first = first_word(&s);
    let second = second_word(&s);

    println!("first word: {}", first);
    println!("second word: {}", second);

    let a: [i32; 20] = [
        1, 2, 3, 4, 5,
        6, 7, 8, 9, 10,
        11, 12, 13, 14, 15,
        16, 17, 18, 19, 20
    ];

    let slice = paginate(&a, 3, 5);
    print_slice(slice);
}

fn create_string() -> String {
    println!("Please provide a string!");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_owned();
}

fn print_string(val: String) {
    println!("{}", val);
}

fn print_and_return(val: String) -> String {
    println!("Printing {}...", val);
    val
}

// reference parameter = borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
}

// mutable reference parameter
fn change_string(s: &mut String) {
    s.push_str(", world!");
}

// slice reference
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut counter = 0;
    let mut first = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if counter < 1 {
                println!("second_word() first space");
                first = i + 1;
                counter += 1;
            } else {
                println!("second_word() next space");
                return &s[first..i];
            }
        }
    }

    &s[first..]
}

fn paginate(a: &[i32], page: usize, size: usize) -> &[i32] {
    println!("paginating with page: {}, size: {}", page, size);
    &a[(page - 1) * size..(page - 1) * size + size]   
}

fn print_slice(a: &[i32]) {
    for item in a.iter() {
        println!("slice item: {}", item);
    }
}