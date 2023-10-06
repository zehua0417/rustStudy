/**
 * @file main.rs
 * @author lihuax
 * @version 0.1
 * @description
 */

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1,101);
    //println!("the secret number is {}", secret_number);

    loop{
        println!("guess a number: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("无法读取行");

        println!("the number you guess is: {}", guess);

        // shadow
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

