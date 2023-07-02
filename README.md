# Hangman Rust
* So i wanted to make an beginner project for rust so i looked some example projects up in the
internet and many people recommendet to programm the hangman game in the terminal
* Now i want to the programm step by step not just for other people but because i think its a great way to revise everything through

* I had some difficulties with errorhandling in rust but overall it was a great little experience

## Code

1. Lets beginn with the libaries 
    * I used two packages the first one **std::io::{self, Write}** was the standart input and output function like 
    ```rust
    // I use this function almost in every programm
    fn readline(input: &mut String){
            stdin()
            .read_line(input)
            .expect("Something went wrong");
        }

        ```
    * The second one use **rand::Rng** is for the random name guessing

    ```rust
    secret_word = rand::thread_rng() // The variabel secret_word gets a function in the libary of rand
        .smaple_iter(&Alphanumeric) //Generates an iterator for a random alphanumeric string
        .take(7) //How long will the string be 
        .map(char::from)//The methode map gets an iterator and can perforome transformation from iterator into char 
        .collect() // Collect() is called in the end of a function to collect all outputs in the function, most of the time this is called when u transform a iterator

2. Now lets beginn with the actuall code, the first to variables takes input from the user in the beginning
    * secret_word makes an random word 
    * An check is transormed so it only can read numbers from the user 
        ```rust 
        let checknumber: u32 = check.trim().parse().expect() //Makes from an readable string to readable int
        ```
    * The vector **guessed_letters** is a mutable Veector that prints all the non guesst characters from a word with an underline _ 
    * The variable **attempts** declares how many attempts you have left but actuall this shodn't be changed because of hangam symbolic ascii there are only 7 avalible 

3. After the welcome message we go into a loop 
    * The user now has to enter his guesses 
    * The mutable variable **guess** first is declared into a string then it the line **io::stdout().flush().unwrap()** lets u input the letter after the message **Enter your gues: **
    * After the user inputs his guess the variable **guess** is transformed into a char with this line **let guess = guess.trim().chars().next()**
4. The **match guess** is actualy just for errorhandling so if the user puts in something other then a string the message **Invalid input: ** shows
    * The **some(letter)** checks if the user did input a word
    * The mutable variable **found** is at the beginning flase becomes the word is not guessed
    * Then we go into a loop where the input of the user is compared to the secret word 
