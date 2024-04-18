use rand::Rng;
use core::num;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess."); // Enviamos informacion a usuario desde consola

        // variables are immutable by default
        // we use mut to set the variable as mutable
        let mut guess = String::new(); // create a variable to store the user input

        io::stdin()
            .read_line(&mut guess) //accept user input.
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
