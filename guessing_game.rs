use std::io;
use rand::Rng;

fn main() {

    println!("GUESS THE NUMBER BETWEEN 1 TO 10!!\n");
    let guess_number = rand::thread_rng().gen_range(1..=10);
    
    let mut count = 1;

    loop {
        eprint!("Type in your Guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You Guessed: {}\n", guess);

        let guess: u32 = guess.trim().parse().expect("Please type in a number!");
        
        if guess > 10 {
            println!("Your guess should be between 1 to 10\n")
        }
        else {
            if guess_number == guess {
                if count == 1 {
                    println!("Very Good you guessed it on first try!!");
                }
                if count == 2 {
                    println!("Good you guessed it on second try!!");
                }
                if count >= 3 {
                    println!("You took multiple guesses but finally guessed it right!!");
                }
                break;
                }
                count += 1;

                if guess_number > guess {
                    println!("Your Guess is too Low");
                }
            else if guess_number < guess {
                println!("Your Guess is too High");
            }
            else {
                println!("No Guess");
            }
        }
    }
}