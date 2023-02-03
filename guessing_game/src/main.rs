use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;
fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    println!("Secret number is {}", secret_number);

    loop {
        println!("Please enter your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_) => continue,
        }; 
        println!("you guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Greater => println!("{}","Too large".red()),
            Ordering::Equal => {
                println!("{}","Correct guess".green());
                break;
            }
        }
    }
}
