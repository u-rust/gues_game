use std::{cmp, io};
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welcome to the guessing game!");
    let secret_number:u8 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please enter your guess (Right guess is {}): ", secret_number);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess:u8 = match guess.trim().parse(){
            Ok(x) => x,
            Err(err) => {println!("Write numbers from 1 to 100");continue;}
        };

        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too small!"),
            Ordering::Greater=>println!("Too big!"),
            Ordering::Equal=>{println!("You win!");break;},
        }
    }
}