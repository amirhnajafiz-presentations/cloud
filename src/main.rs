extern crate rand;

include!("letter/letter.rs");
include!("game/enum.rs");
include!("game/game.rs");
include!("utils/file.rs");
include!("utils/input.rs");

// using rand library
use rand::{Rng, thread_rng};

// importing file and io libraries
use std::fs::File;
use std::io::prelude::*;

use std::io;

const ALLOWED_ATTEMPTS: u8 = 5;

fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);

    println!("Welcome to Hangman!");

    loop {
        println!("\nYou have {} turns left.", turns_left);
        display_progress(&letters);


        println!("\nPlease enter a letter to guess:");
        let user_char = read_user_input_character();

        if user_char == '*' {
            break;
        }

        let mut at_least_on_revealed = false;
        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_on_revealed = true;
            }
        }

        if !at_least_on_revealed {
            turns_left -= 1;
        }

        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nCongrats, you won! The word was {}", selected_word);
                break;
            }
            GameProgress::Lost => {
                println!("\nSorry, you lost!");
                break;
            }
        }
    }

    println!("\nGoodbye!");
}
