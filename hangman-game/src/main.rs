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

    let content = match fs::read_to_string("words.txt") {
        Ok(c) => c,
        Err(_) => {
            eprint!("Error: file not found or cannot be read.");
            return;
        }
    };

    if content.trim().is_empty() {
        eprintln!("Error: The words.txt file is empty!");
        return;
    }

    let words: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    let mut rng = thread_rng();
    let secret_word = words
        .choose(&mut rng)
        .expect("Error: No words available to choose from.");

    let mut correct_letters: HashSet<char> = HashSet::new();
    let mut wrong_letters: HashSet<char> = HashSet::new();

    loop {
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

        let mut player_char_input = String::new();
        println!("Please input your guess: ");
        io::stdin()
            .read_line(&mut player_char_input)
            .expect("Error on reading player char!");

        let player_char_input = player_char_input
            .trim()
            .to_lowercase()
            .chars()
            .next()
            .expect("Error transforming string to char");

        if correct_letters.contains(&player_char_input)
            || wrong_letters.contains(&player_char_input)
        {
            println!("You already guessed that letter!");
            continue;
        }

        if secret_word.contains(player_char_input) && !wrong_letters.contains(&player_char_input) {
            correct_letters.insert(player_char_input);
        } else {
            wrong_letters.insert(player_char_input);
            errors += 1;
            eprintln!("Wrong guess!");
        }

        if correct_letters == secret_word.chars().collect() {
            println!("{}\nYou win!", secret_word);
            break;
        }

        if errors == attempts {
            println!(
                "{}\nYou lose! The secret word is: {}",
                STATUS[errors], secret_word
            );
            break;
        }
    }
}
