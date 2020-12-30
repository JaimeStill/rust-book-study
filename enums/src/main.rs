#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message: {:#?}", self);
    }

    fn try_call(opt: &Option<Message>) {
        match opt {
            Option::Some(val) => val.call(),
            Option::None => println!("Message does not have a value!"),
        }
    }

    fn handle_message(&self) {
        if let Message::Quit = self {
            println!("Terminating...")
        } else {
            self.call()
        }
    }
}

fn get_u8() -> u8 {
    loop {
        println!("Guess one of the lucky numbers from 0 - 255");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        return input;
    }
}

fn enums_intro() {
    println!("Running enums_intro()");

    let w = Message::Write(String::from("hello"));
    w.call();
    println!();

    let q = Message::Quit;
    q.call();
    println!();

    let m = Message::Move { x: 32, y: 64 };
    m.call();
    println!();

    let c = Message::ChangeColor(0, 255, 64);
    c.call();
    println!();
}

fn explore_option() {
    /*
        The standard library Option enum
        is useful for encoding the scenario
        where an option could have a value or
        or could be null.
    */

    /*
        enum Option<T> {
            Some(T),
            None
        }
    */

    println!("Running explore_option()");
    let message = Some(Message::Write(String::from("Has a value")));
    Message::try_call(&message);
    println!();

    let empty: Option<Message> = None;
    Message::try_call(&empty);
    println!();
}

fn match_placeholder() {
    println!("Running match_placeholder()");

    match get_u8() {
        3 => println!("Three is lucky!"),
        6 => println!("Six is lucky!"),
        9 => println!("Nine is lucky!"),
        _ => ()
    }

    println!();
}

fn message_if_let() {
    println!("Running message_if_let()");

    let message = Message::Write(String::from("Meet me in outerspace"));
    message.handle_message();

    let message = Message::Quit;
    message.handle_message();
    println!();
}

fn main() {
    enums_intro();
    explore_option();
    match_placeholder();
    message_if_let();
}
