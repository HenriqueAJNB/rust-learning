use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::fs;
use std::io;

const STATUS: [&str; 7] = [
    r"_____________
|            |
|            
|           
|           
|=====================",
    r"_____________
|            |
|            O
|           
|           
|=====================",
    r"_____________
|            |
|            O
|            |
|           
|=====================",
    r"_____________
|            |
|            O
|           /|
|           
|=====================",
    r"_____________
|            |
|            O
|           /|\
|           
|=====================",
    r"_____________
|            |
|            O
|           /|\
|           / 
|=====================",
    r"_____________
|            |
|            O
|            -
|           /|\
|           / \
|=====================",
];

fn main() {
    let mut errors: usize = 0;
    let attempts: usize = 6;

    let content: String = read_words_file();

    let words: Vec<String> = transform_content_on_string_vectors(content);

    let secret_word: &String = pick_random_word(&words);

    let mut correct_letters: HashSet<char> = HashSet::new();
    let mut wrong_letters: HashSet<char> = HashSet::new();

    loop {
        print_game_status(secret_word, &correct_letters, &wrong_letters, errors);

        let player_char_input: char = read_char_guess_from_player();

        let char_already_guessed: bool = correct_letters.contains(&player_char_input)
            || wrong_letters.contains(&player_char_input);

        if char_already_guessed {
            println!("You already guessed that letter!");
            continue;
        }

        let correct_guess: bool = secret_word.contains(player_char_input) && !wrong_letters.contains(&player_char_input);

        if correct_guess {
            correct_letters.insert(player_char_input);
        } else {
            wrong_letters.insert(player_char_input);
            errors += 1;
            println!("Wrong guess!");
        }

        let win_game: bool = correct_letters == secret_word.chars().collect();

        if win_game {
            println!("{}\nYou win!", secret_word);
            break;
        }

        let lose_game: bool = errors == attempts;

        if lose_game {
            println!(
                "{}\nYou lose! The secret word is: {}",
                STATUS[errors], secret_word
            );
            break;
        }
    }
}

fn read_words_file() -> String {
    let content = match fs::read_to_string("words.txt") {
        Ok(c) => c,
        Err(_) => {
            panic!("Error: file not found or cannot be read.");
        }
    };

    if content.trim().is_empty() {
        panic!("Error: The words.txt file is empty!");
    }
    content
}

fn transform_content_on_string_vectors(content: String) -> Vec<String> {
    content.lines().map(|s| s.to_string()).collect()
}

fn pick_random_word(words: &Vec<String>) -> &String {
    let mut rng = thread_rng();
    words
        .choose(&mut rng)
        .expect("Error: No words available to choose from.")
}

fn print_game_status(
    secret_word: &String,
    correct_letters: &HashSet<char>,
    wrong_letters: &HashSet<char>,
    errors: usize,
) {
    let mut encoded_word = String::new();
    for character in secret_word.chars() {
        if correct_letters.contains(&character) {
            encoded_word.push(character)
        } else {
            encoded_word.push('_')
        }
    }

    println!("{}", STATUS[errors]);
    println!("{}", encoded_word);
    println!("\nWrong letters: {:?}", wrong_letters);
    println!("\nCorrect letters: {:?}\n", correct_letters);
}

fn read_char_guess_from_player() -> char {
    let mut player_char_input = String::new();
    println!("Please input your guess: ");
    io::stdin()
        .read_line(&mut player_char_input)
        .expect("Error on reading player char!");

    player_char_input
        .trim()
        .to_lowercase()
        .chars()
        .next()
        .expect("Error transforming string to char")
}
