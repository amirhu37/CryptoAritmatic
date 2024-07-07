use itertools::Itertools;
use std::process::Command;
use std::collections::HashSet;



/// you can add a massage to show as a hint 
#[macro_export]
macro_rules! input {
    ( $input_type:ty ) => {
        input!($input_type, "");
    };

    ( ) => {
        input!(String, "");
    };

    (  $msg:expr ) => {
        input!(String, $msg)
    };

    ( $input_type:ty, $msg:expr ) => {{
        use std::io::{self, Write};
        print!("{}", $msg  );
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");
        let trimmed_input = user_input.trim().to_string();
        match trimmed_input.parse::<$input_type>() {
            Ok(r) => r,
            Err(error) => panic!("Cannot parse: {:?}", error),
        }
    }};
}


/// ## word_to_number
/// ### inputs : 
///          word : &str = the word 
///          mapping : &[(char, u32)] = tuple of chr and it's number 
/// ----
/// Helper function to convert a word to its numerical value based on current assignment
fn word_to_number(word: &str, mapping: &[(char, u32)]) -> u32 {
    let mut number: u32 = 0;
    for c in word.chars() {
        let digit: u32 = mapping.iter().find(|&&(ch, _)| ch == c).unwrap().1;
        number = number * 10 + digit;
    }
    number
}

/// ## is_valid_solution
/// ### inputs : 
///         words: &Vec<String> = the word 
///         result: &String = the Result Word
///         mapping : &[(char, u32)] = tuple of chr and it's number 
/// ### output :
///         boolean that check if the solution valid
/// ---
/// Function to check if current mapping satisfies the puzzle
fn is_valid_solution(words: &Vec<String>, result: &String, mapping: &[(char, u32)]) -> bool {
    let sum: u32 = words.iter().map(|word| word_to_number(word.as_str(), mapping)).sum();
    sum == word_to_number(result, mapping)
}

/// ## solve_crypto_arithmetic
/// #### inputs : 
///         words: &Vec<String> = the word 
///         result: &String = the Result Word
/// #### output :
///     tuple of (chr , index)
/// ---
/// Function to generate permutations of digits and find a valid solution
fn solve_crypto_arithmetic(words: Vec<String>, result: String) -> Option<Vec<(char, u32)>> {
    let mut letters: HashSet<char> = HashSet::new();
    for word in words.iter().chain(std::iter::once(&result)) {
        for c in word.chars() {
            letters.insert(c);
        }
    }
    let letters: Vec<char> = letters.into_iter().collect();
    // check if there is more than 10 letter. then we are sure there won't solution
    if letters.len() > 10 {
        return None; // More than 10 unique letters, no solution possible with single digits
    }

    let digits: Vec<u32> = (0..10).collect();
    let permutations: itertools::Permutations<std::iter::Cloned<std::slice::Iter<u32>>> = digits.iter().cloned().permutations(letters.len());

    for perm in permutations {
        let mapping: Vec<(char, u32)> = letters.iter().cloned().zip(perm).collect();
        if is_valid_solution(&words, &result, &mapping) {
            return Some(mapping);
        }
    }
    None
}



/// #### output:
///     first a vector of two word
///     second a string thst is output
/// ---
/// helper for get 3 words for processes
fn inputs() -> (Vec<String>, String) {
    let input1: String = input!("Two Words as Input, Separated with Whitespace? ");
    let words: Vec<String> = input1.split_whitespace().map(String::from).collect();
    
    let result: String = input!("Result String? ");
    
    (words, result)
}


/// just for cleaning Terminal 
/// Work only for Windows
fn cls() {
    Command::new("cmd")
        .args(&["/C", "cls"])
        .status()
        .unwrap();
}

fn main() {
    cls();
    // get input words
    let ( words, result) = inputs();
    // Print what we got as String Words
    println!("{} + {} = {}", words[0], words[1], result);
    // get solution
    match solve_crypto_arithmetic(words, result) {
        Some(mapping) => {
            // Extract characters and form a string
            let extracted_string: String = mapping.iter().map(|&(ch, _)| ch).collect();
            println!("Solution found: {} " , extracted_string);
            for (ch, digit) in mapping {
                println!("{} = {}", ch, digit);
            }
        }
        None => println!("No solution found."),
    }
}



