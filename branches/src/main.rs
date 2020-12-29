use std::io;

fn main() {
    let input = get_input();
    run_if(input);
    run_if_let(input);
    run_loop(input);
    run_while(input);
    run_array_loop();
    run_array_rev();
}

fn run_if(x: i32) {
    if x % 4 == 0 {
        println!("{} is divisible by 4", x);
    } else if x % 3 == 0 {
        println!("{} is divisible by 3", x);
    } else if x % 2 == 0 {
        println!("{} is divisible by 2", x);
    } else {
        println!("{} is not divisible by 4, 3, or 2", x);
    }
}

fn run_if_let(x: i32) {
    let result = if x <= 100 { true } else { false };

    println!("x is less than or equal to 100 is: {}", result);
}

fn run_loop(x: i32) {
    if x != 0 {
        let mut counter = x;

        let result = loop {
            counter = if counter > 0 {
                counter - 1
            } else {
                counter + 1
            };

            if counter == 0 {
                break "success";
            }
        };

        println!("{}: looped through {} times", result, x);
    } else {
        println!("Unable to loop through {}", x);
    }
}

fn run_while(x: i32) {
    let mut counter = x;
    while counter != 0 {
        counter = if x < 0 { counter + 1 } else { counter - 1 };
        println!("counter: {}", counter);
    }
}

fn run_array_loop() {
    let arr = [3, 5, 7, 9];

    for val in arr.iter() {
        println!("val: {}", val);
    }
}

fn run_array_rev() {
    for number in (1..4).rev() {
        println!("{}", number);
    }

    println!("LIFTOFF!");
}

fn get_input() -> i32 {
    loop {
        println!("Please input a number");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    }
}
