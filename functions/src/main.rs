// a single-line comment
fn main() {
    print_integer(five());
    print_integers(12, 8);
    express();
    print_integer(plus_one(6));
}

/*
    Multi-line comments
    are created in 
    this way.
*/
fn print_integer(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_integers(x: i32, y: i32) {
    println!("x: {}, y: {}", x, y);
}

fn five() -> i32 {
    5
}

fn express() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}