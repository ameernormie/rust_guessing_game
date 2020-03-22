use rand::Rng;
use std::cmp::Ordering;
use std::io;

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100 {}", value);
        }

        Guess { value }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess.");
    loop {
        // Creates a mutable variable that is currently bound to a new, empty instance of a String.
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        /* we want to convert the String the program reads as input into a real number
        type so we can compare it numerically to the secret number.
        Rust allows us to shadow the previous value of gue  ss with a new one.
        This feature is often used in situations in which you want to convert a value from one type to another type.
        The parse method on strings parses a string into some kind of number.
        The parse method returns a Result type. If parse returns an Err Result variant because it couldn’t
        create a number from the string, the expect call will crash the game and print the message we give it.
        */
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => Guess::new(num).value(),
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };
        println!("You guessed: {}", guess);
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
