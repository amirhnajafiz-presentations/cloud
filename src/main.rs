// importing file and io libraries
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let selected_word = select_word();
}

fn select_word() -> String {
    /* Open file */
    let mut file = File::open("words.txt").expect("couldn't open file!");

    /* Load file contents */
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("an error occurred while reading file!");



}
