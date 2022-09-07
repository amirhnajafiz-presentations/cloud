fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    let mut all_revealed = true;
    for letter in letters {
        if !letter.revealed {
            all_revealed = false;

            break;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    if turns_left > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:");

    /* Display appropriate character */
    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}