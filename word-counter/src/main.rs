use std::collections::HashMap;
use std::io;

fn main() {

    // Initialize variable with input from std using the block as expression
    let words = {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input lines.");

        input
    };

    let word_count = count_words(&words);

    println!("Word count is:\n{word_count:#?}");
}

fn count_words(words: &str) -> HashMap<&str, u32> {
    let mut word_count: HashMap<&str, u32> = HashMap::new();
    for word in words.split_whitespace() {
        word_count.entry(word).and_modify(|counter| *counter += 1).or_insert(1);
    }

    word_count
}
