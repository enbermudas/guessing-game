use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number!\n---------------------------");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut tries: u32 = 0;

    // println!("Pssst, the secret number is: {}", secret_number);

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                tries += 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                tries += 1;
            },
            Ordering::Equal => {
                println!("You win!");
                let word = if tries > 1 { "tries" } else { "try" };
                println!("It took you {} {} to win!", tries, word);
                break;
            },
        }
    }
}
