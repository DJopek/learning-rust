use std::io;

fn main() {


    for _i in 1..=3 {
        println!("{}", _i);

        println!("Guess the number!");

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

    println!("You guessed: {guess}");

    }
    
}