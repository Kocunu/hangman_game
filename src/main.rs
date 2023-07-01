// TODO:
// [x] Hangman Symbolic
// [ ] Programming design
use std::io::{self, Write, stdin};
use rand::{distributions::Alphanumeric, Rng}; 

fn main() {
    let mut secret_word = String::new();
    let mut check = String::new();
    println!("1. Guess your own word");
    println!("2. Let me generate a random word for you");
    readline(&mut check);

    let check_number: u32 = check.trim().parse().expect("Something went wrong");

    if check_number == 1 {
        println!("Give in the word u want to guess");
        readline(&mut secret_word);
    } else {
        secret_word = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
    }

    let mut guessed_letters: Vec<char> = vec!['_'; secret_word.len()-1];
    let mut attempts = 7;

    println!("Welcome to Hangman!");

    loop {
        println!("\nAttempts left: {}", attempts);
        println!("Guessed letters: {}", guessed_letters.iter().collect::<String>());

        let mut guess = String::new();
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim().chars().next();

        match guess {
            Some(letter) => {
                let mut found = false;
                for (i, c) in secret_word.chars().enumerate() {
                    if c == letter {
                        guessed_letters[i] = letter;
                        found = true;
                    }
                }

                if !found {
                    attempts -= 1;
                    println!("Incorrect guess!");
                    if attempts == 6 {println!("
                                               '''
  +---+
  |   |
      |
      |
      |
      |
========='''");
                    } else if attempts == 5 {
                        println!("'''
  +---+
  |   |
  O   |
      |
      |
      |
========='''");
                    } else if attempts == 4 {
                        println!("'''
  +---+
  |   |
  O   |
  |   |
      |
      |
========='''");
                    } else if  attempts == 3 {
                        println!("'''
  +---+
  |   |
  O   |
 /|   |
      |
      |
========='''");
                    } else if attempts == 2 {
                        println!(r"'''
  +---+
  |   |
  O   |
 /|\  |
      |
      |
========='''");
                    } else if attempts == 1 {
                        println!(r"'''
  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
========='''");
                    } else if attempts == 0 {
                        println!(r"'''
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
========='''");
                    } {

                    }
                }
            },
            None => {
                println!("Invalid input!");
                continue;
            }
        }

        if guessed_letters.iter().all(|&c| c != '_') {
            println!("Congratulations! You guessed the word: {}", secret_word);
            break;
        }

        if attempts == 0 {
            println!("Game over! The word was: {}", secret_word);
            break;
        }
    }
}
fn readline (input: &mut String) {
    stdin()
        .read_line(input)
        .expect("Something went wrong");
}
