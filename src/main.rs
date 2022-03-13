use rand::Rng;
use std::cmp::Ordering;
use std::io;
use ansi_term::Colour;

fn main() {
    println!("Guess the number!");
    
    //pick a random number
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut tries = 0;

    loop {
        let mut guess = String::new();
        //read the guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // the guess is in  a string format, so well need to turn that into an int.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //increment the number of tries
        tries = tries + 1;
        //if the user took more than 10 tries, they failed
        if tries > 10 {
            println!("{} the number was {}", Colour::Red.paint("you lose!"), secret_number);
            break;
        }
        //tell the user how far off they were
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", Colour::Yellow.paint("Too small!")),
            Ordering::Greater => println!("{}", Colour::Yellow.paint("Too big!")),
            Ordering::Equal => {
                println!("{} it took you {} tries.", Colour::Green.paint("you win!"), tries,);
                break;
            }
        }
    }
}
