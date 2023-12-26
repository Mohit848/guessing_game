use std::io;
use std::cmp::Ordering;
use colored::*;
use rand::Rng;
fn main() {

    loop {
    let secret_number= rand::thread_rng().gen_range(1, 11);

    println!("Hi Please guess the number");
        let mut guess: String = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=> num,
            Err(_)=>continue,
        };
        println!("You guessed: {}", guess);
        println!("Secret number is:  {}", secret_number);
        match guess.cmp(&secret_number) {
            Ordering::Equal=>{
                println!("{}","You win!!".green());
                break;
            },
            Ordering::Greater => println!("{}","Too Big!".red()),
            Ordering::Less => println!("{}","Too small!".red()),
        }
    }
    
}
