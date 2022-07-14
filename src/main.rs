use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("=== Guessing Game! ===\n");
  
  let secret_number = rand::thread_rng().gen_range(1..=100);
  let mut attempts: u8 = 5;

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
        println!("Please! Type a number between 1 and 100!\n");
        continue;
      }
      num  
    },
    Err(_) => {
      println!("Please! Type a number!\n");
      continue;
    }
  };

  match guess.cmp(&secret_number) {
    Ordering::Less => {
      println!("Too small!\n");
      attempts -= 1;
    },

    Ordering::Greater => {
      println!("Too big!\n");
      attempts -= 1;
    },

    Ordering::Equal => {
      println!("You Win!");
      break;
    }

 }
   

}




}
