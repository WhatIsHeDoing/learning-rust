use rand::Rng;
use std::{cmp::Ordering, io};

pub enum WebEvent {
    Click { x: i64, y: i64 },
    KeyPress(char),
    PageLoad,
    PageUnload,
    Paste(String)
}

/// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
pub fn enums(event: WebEvent) -> String {
    match event {
        WebEvent::Click { x, y } => format!("clicked at x={}, y={}.", x, y),
        WebEvent::KeyPress(c) => return format!("pressed '{}'.", c),
        WebEvent::PageLoad => return String::from("page loaded"),
        WebEvent::PageUnload => return String::from("page unloaded"),
        WebEvent::Paste(s) => return format!("pasted \"{}\".", s)
    }
}

#[cfg(test)]
mod enums_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(enums(WebEvent::PageLoad), "page loaded");
    }
}

/// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
pub fn secret_number() {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
