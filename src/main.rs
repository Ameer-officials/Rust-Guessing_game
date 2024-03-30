use rand::Rng;
use std::{io, num::ParseIntError, u32};

fn main() {

    let mut rng = rand::thread_rng();
    let number : u32 = rng.gen_range(1..=10);
    println!("System has generated a number between 1 to 10");
    println!("You have 3 choice's to guess the number");

    let mut _int :u32 = 0;

    let mut already_guesses_numbers : Vec<u32> = Vec::new();

    while _int < 3{
        let mut guessed_number : String= String::new();
        io::stdin().read_line(&mut guessed_number).expect("Unable to detect the number");
        let guessed_number: Result<u32, ParseIntError> = guessed_number.trim().parse();


        match guessed_number {
            Ok(guess)=>{
                if already_guesses_numbers.contains(&guess){
                    println!("You have already guessed this number, {{{}}}. Please guess other than this number", guess);
                    continue;
                };
                already_guesses_numbers.push(guess);
                if guess == number{
                    println!("Hurry, you guessed correct number. 🎉🎉Congrtulations");
                    println!("The guessed number is {}" , &guess);
                    println!("System generated number is {}" , &number);
                    break;
                } else if _int == 2 {
                    println!("No more guesses , LOOOOOOOOOOOOOOSER");
                    println!("System generated number is {}" , &number);
                    _int += 1;
                }else{
                    println!("Wrong guess! Try again.");
                    _int += 1;

                }
            }
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
            }
        }
    }
}
