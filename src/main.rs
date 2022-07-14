use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("=== Guessing Game! ===\n");
  
  let secret_number = rand::thread_rng().gen_range(1..=100);
  let mut attempts: u8 = 10;

loop {
  if attempts == 0 {
    println!("You have {attempts} attempts left... You lose!");
    break;
  }

  println!("You have {attempts} attempts. Guess a number between 1 and 100:");
  let mut guess = String::new();
  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
  
  let guess: u32 = match guess.trim().parse() {
    Ok(num) => {
      if num > 100 || num < 1 {
        println!("Please! Type a number between 1 and 100!");
        continue;
      }
      num  
    },
    Err(_) => {
      println!("Please! Type a number!");
      continue;
    }
  };
   

}




}
