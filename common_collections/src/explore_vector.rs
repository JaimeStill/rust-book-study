#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

pub fn run() {
    // initialize a mutable empty Vec<T>
    let mut v: Vec<u8> = Vec::new();

    /*
        initialize a Vec<T> with implicit generic type
        using the vec! macro. The implicit type is i32
        because that's Rust's default integer type
    */
    let _v2 = vec![1, 2, 3];

    /*
        initialize a Vec<T> with an explicit generic
        type using the vec! macro.
    */
    let _v3: Vec<u32> = vec![100, 200, 300];

    // Add values to a vector
    v.push(6);
    v.push(9);

    // Add a range of values to a vector
    v.extend_from_slice(&[4, 2, 0]);

    println!("Resulting vector:\n{:#?}", v);
    println!();

    // reading elements of vectors
    let third: &u8 = &v[2];
    println!("The third element is {}", third);

    match v.get(3) {
        Some(fourth) => println!("The fourth element is {}", fourth),
        None => println!("There is no fourth element.")
    }

    println!();

    // iterating over vector values
    for i in &v {
        println!("{}", i);
    }

    println!();

    // make changes to all elements in the vector

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    println!();

    // using an enum to store multiple values
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    for (i, val) in row.iter().enumerate() {
        match val {
            SpreadsheetCell::Int(cell) => println!("Spreadsheet cell {}: int = {}", i + 1, cell),
            SpreadsheetCell::Float(cell) => println!("Spreadsheet cell {}: float = {}", i + 1, cell),
            SpreadsheetCell::Text(cell) => println!("Spreadsheet cell {}: text = {}", i + 1, cell)
        }
    }

    println!();
}