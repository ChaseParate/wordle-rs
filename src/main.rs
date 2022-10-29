use std::fs;
use std::io::{self, Write};
use std::iter::zip;

use rand::{seq::SliceRandom, thread_rng};

const ALL_WORDS_FILENAME: &str = "all_words.txt";
const CHOSEN_WORDS_FILENAME: &str = "chosen_words.txt";

const NUM_GUESSES: u32 = 6;

fn read_file_lines(filename: &str) -> Vec<String> {
    let file_contents = fs::read_to_string(filename).expect("Failed to read file");
    file_contents
        .lines()
        .map(|str| str.trim().to_string())
        .collect()
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from standard input");

    input.trim().to_string()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Hint {
    Correct,
    Close,
    Wrong,
}

fn generate_hints(guess: &str, chosen_word: &str) -> [Hint; 5] {
    let mut hints = [Hint::Wrong; 5];

    // Check for correctly placed letters
    let mut remaining_characters: Vec<char> = Vec::with_capacity(5);
    for (index, (guess_character, correct_character)) in
        zip(guess.chars(), chosen_word.chars()).enumerate()
    {
        if guess_character == correct_character {
            hints[index] = Hint::Correct;
        } else {
            remaining_characters.push(correct_character);
        }
    }

    // For remaining incorrect letters, check if they are in the wrong position
    for (index, guess_character) in guess.chars().enumerate() {
        if hints[index] == Hint::Correct {
            continue;
        }

        if remaining_characters.contains(&guess_character) {
            hints[index] = Hint::Close;

            let char_position = remaining_characters
                .iter()
                .position(|char| *char == guess_character)
                .unwrap();
            remaining_characters.remove(char_position);
        }
    }

    hints
}

fn main() {
    let chosen_words = read_file_lines(CHOSEN_WORDS_FILENAME);
    let all_words = read_file_lines(ALL_WORDS_FILENAME);

    let mut rng = thread_rng();
    let chosen_word = chosen_words.choose(&mut rng).unwrap();

    // println!("[DEBUG] The word is \"{}\".", chosen_word);
    // println!();

    for guess_number in 1..=NUM_GUESSES {
        // Get and validate guess
        let mut guess: String;
        loop {
            print!("Guess {}: ", guess_number);
            io::stdout().flush().unwrap();

            guess = get_input().to_lowercase();

            if all_words.binary_search(&guess).is_ok() {
                break;
            } else {
                println!("Invalid guess");
            }
        }

        // Generate and print hints
        let hints = generate_hints(&guess, chosen_word);
        println!("{:?}", hints);

        // Check for win
        if hints.iter().all(|hint| *hint == Hint::Correct) {
            println!("You win!");
            break;
        }
    }

    println!("The word was \"{}\"!", chosen_word);
}
