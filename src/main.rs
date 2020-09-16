use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut _count = 0;
    let value = 4;
    println!("Guess the number! You have {} attempts to guess the number", value);

    

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        _count +=1;
        let  values = value - _count;
        println!("You guessed: {}", guess);
       
        
        if _count < value {
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!\n{} attempts left",values),
                Ordering::Greater => println!("Too big!\n{} attempts left", values),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        } else{
            println!("The attempts ended {}", values);
            println!("The secret value is {}",secret_number);
            break;
        }
        
    }
}
