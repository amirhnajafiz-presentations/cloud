fn select_word() -> String {
    /* Open file */
    let mut file = File::open("words.txt").expect("couldn't open file!");

    /* Load file contents */
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("an error occurred while reading file!");

    /* Get individual words */
    let available_words: Vec<&str> = file_contents.trim().split(",").collect();

    /* Generating random index */
    let random_index = thread_rng().gen_range(0, available_words.len());

    return String::from(available_words[random_index]);
}