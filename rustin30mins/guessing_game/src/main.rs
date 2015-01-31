
use std::cmp::Ordering;
use std::old_io;
use std::rand;

fn main() {

    let number = (rand::random::<u32>() % 100) + 1;

    println!("Guess a number!");

    loop {
        println!("Please input your guess:");

        let input = old_io::stdin().read_line()
                        .ok()
                        .expect("Failed to read line...");

        let input_num: Option<u32> = input.trim().parse();

        let num = match input_num {
            Some(num) => num,
            None => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("You guessed: {}", input);

        match cmp(num, number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Bingo!");
                return;
            }
        }       
    }

}

fn cmp(a: u32, b: u32) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
