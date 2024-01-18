use std::io;
use std::io::Write;
use rand::seq::IteratorRandom;
use rand::thread_rng;

fn main() {
    let mut password_length: String = Default::default();
    loop {
        password_length.clear();
        print!("Enter password length: ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut password_length);
        let my_number = password_length.trim().parse::<i32>();
        if !my_number.is_ok() {
            println!("Please enter a number!");
            continue;
        }
        else {
            if !(5..=50).contains(&my_number.clone().unwrap()) {
                println!("Please enter a number in 5 to 50 range!");
                continue;
            }
            break;
        }
    }
    let password_length = password_length.trim().parse::<i32>().unwrap();
    println!("Your password is: {}", generate_random_password(&password_length));
}

fn generate_random_password(password_length: &i32) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789@!£#&*$%";
    let random_chars: String = chars.chars().choose_multiple(&mut thread_rng(), *password_length as usize).into_iter().collect();
    return random_chars;
}
