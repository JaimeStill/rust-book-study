use std::error::Error;

mod recoverable;
mod unrecoverable;

fn main() -> Result<(), Box<dyn Error>> {
    /*
        To see full backtrace:
        RUST_BACKTRACE=1 cargo run

        uncomment to see effects of panic
    */
    // unrecoverable::_run_panic();
    
    let f = recoverable::read_with_match("hello.txt");
    println!("{:#?}", f);

    let f = recoverable::read_with_unwrap("hello.txt");
    println!("{:#?}", f);

    let f = recoverable::run_with_expect("hello.txt");
    println!("{:#?}", f);

    match recoverable::read_with_propogation("world.txt") {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{:#?}", e)
    };

    let f = recoverable::read_string("hello.txt")?;
    println!("{}", f);

    Ok(())
}