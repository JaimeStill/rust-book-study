pub fn run() {
    // creating a new empty string
    let mut s = String::new();

    // updating a string with a string
    s.push_str("hello");

    // adding a character to a string
    s.push(' ');

    s.push_str("world");
    
    println!("{}", s);
    println!();

    // creating from a literal string
    let _lit_s = "initial contents".to_string();

    // strings are UTF-8 encoded, so can consist of any properly encoded data
    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);
    let hello = String::from("👋");
    println!("{}", hello);
    println!();

    // concatenation with the + operator or the format! macro
    let s1 = String::from("Greetings, ");
    let s2 = String::from("Jaime!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    println!();

    /*
        String does not support indexing by integers.
        let s = String::from("hello");
        let h = s[0]; // THIS IS NOT ALLOWED

        Instead, you must specify the range of bytes you want.
    */

    let hello = String::from("hello");
    let hell = &hello[..=3];
    println!("&hello[..=3] = {}", hell);
    println!();

    // iterating through a string
    for c in "Jaime".chars() {
        println!("{}", c);
    }
    println!();

    for c in "Jaime".bytes() {
        println!("{} = {}", char::from(c), c);
    }

    println!();
}