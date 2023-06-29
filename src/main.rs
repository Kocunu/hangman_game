use std::io::stdin;
use std::io::stdout;
use std::io::Write;

fn main() {
    //Introduction
    let mut check: u32 = 0;
    let mut input: u32;
    let (mut word, mut guess, mut user_input,mut guess_char) = (String::new(),String::new(),String::new(),String::new());

    println!("Enter the word u want the guess: ");
    readline(&mut word);
    println!(" ");

    for _ in 0..word.len() {
        print!("_");
    }
    /////////////////
    loop{
        println!(" ");
        println!("1: Do u want to guess a character ");
        println!("2: Do u want to guess the word ");
        readline(&mut user_input);
        input = user_input.trim().parse().expect("Please give in a number");

        if input == 1 {
            loop {
                let word_char: Vec<char> = word.chars().collect(); // Spliting Char chunks in array
                readline(&mut guess_char);
                let guessing_char = guess_char.chars().next(); // Reading first character of a String

                if let Some(guessing_character) = guessing_char{
                    for (i, char) in word_char.iter().enumerate() {
                        if *char == guessing_character {
                            println!("Du hast {} richtig geraten an der Stelle {}", i, char);
                            check = 0;
                        } else {
                            println!("Diesen Buchstaben gibt es nicht");
                            break;
                        }
                    }
                }
                
                break
            }

        } else if input == 2 {
            println!("Give your guess in");
            readline(&mut guess);
            match guess {
                _ if guess == word =>{
                    println!("You win!!");
                    break;
                },
                _ => {
                    println!("Try again");
                    check = 5;
                    break;
                }
            }
        } else {
            println!("Please give either 1 or 2");
        }
    }

    match check {
        0 => println!("Good Job"),
        1 => println!("L"),
        2 => println!("LO"),
        3 => println!("LOS"),
        4 => println!("LOSE"),
        5 => println!("LOSER \n You have LOST"),
        _ => println!("Something went wrong"),
    }
}

fn readline(input: &mut  String) {
    stdout().flush().expect("Faild to read line");
    stdin().read_line(input).expect("Faild to read line");
}

fn wrong_guess() {

}

fn compare() {

}

/*
    readline(&mut guess);
    */
