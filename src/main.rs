extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("=== Guessing Game ===");

    let secret = rand::thread_rng().gen_range(1, 101);
    // TODO: only for debug target. not for optimized release
    println!("secret is {}", secret);

    loop {
        println!("Enter a guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        // match io::stdin().read_line(&mut guess) {
        //     Ok(n) => {
        //         println!("{} bytes read", n);
        //         println!("You guessed: {}", guess);
        //     }
        //     Err(error) => println!("Failed to read line: {}", error),
        // }

        // assert!(io::stdin().read_line(&mut guess).is_ok());
        // println!("you guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("you guessed less"),
            Ordering::Equal => {
                println!("you win the internets");
                break
            },
            Ordering::Greater => println!("you guessed more")
        }
    }
}
