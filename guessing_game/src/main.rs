use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is : {}", secret_number);
    loop{
    println!("Enter a number you guess.");

    let mut guess = String::new();

    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read");
    
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,  
     };
    println!("You guessed: {}", guess);
    

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too less!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {println!("Guessed right!");
                            break;
                            
                         }
       
    }
    }
}