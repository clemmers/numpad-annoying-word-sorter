use std::collections::HashMap;
use std::time::Instant;

fn main() -> std::io::Result<()> {

    // start measuring time for loading dictionary of words
    let start_time = Instant::now();

    let dickt = std::fs::read_to_string("./list_of_words.txt")
        .expect("uh oh something happened when trying to read the word file...");

    let words = dickt.split_whitespace().collect::<Vec<_>>();

    // stop measuring time for loading dictionary of words
    let end_time = Instant::now();
    let duration = end_time - start_time;

    println!("loading dictionary of words took {:?}", duration);

    // start measuring time for rating all words
    let start_time = Instant::now();

    let keyboard_vec = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let mut keyboard_layout: HashMap<char, &str> = HashMap::new();
    for string in keyboard_vec
    {
        for c in string.chars()
        {
            keyboard_layout.insert(c, string);
        }
    }


    let mut words_rated: Vec<(String, i8)> = Vec::new();
    for word in &words
    {
        words_rated.push((word.to_string(), how_hard_is_word_to_type(&word, &keyboard_layout)));
    }

    // stop measuring time for rating all words
    let end_time = Instant::now();
    let duration = end_time - start_time;

    println!("rating words took {:?}", duration);

    // start measuring time for sorting
    let start_time = Instant::now();

    words_rated.sort_by(|a, b|
    {
        match b.1.cmp(&a.1)
        {
            std::cmp::Ordering::Equal => b.0.len().cmp(&a.0.len()),
            ord => ord,
        }
    });

    // stop measuring time for sorting
    let end_time = Instant::now();
    let duration = end_time - start_time;

    println!("sorting took {:?}", duration);

    // start measuring time for writing to file
    let start_time = Instant::now();

    //let mut file = std::fs::File::create("output.txt")?;

    let mut output = String::new();
    for pair in &words_rated
    {
        output.push_str(&pair.0);
        output.push_str("\n");
    }

    std::fs::write("output.txt", output)?;
    // stop measuring time for sorting and writing to file
    let end_time = Instant::now();
    let duration = end_time - start_time;

    println!("writing to file took {:?}", duration);

    Ok(())

}

// returns an int that describes how annoying a word is to type on a numeric keypad
// by determining how many times there are two consecutive same-key letters
fn how_hard_is_word_to_type(word: &str, keyboard_layout: &HashMap<char, &str>) -> i8
{
    let chars: Vec<_> = word.chars().collect();
    let mut difficulty: i8 = 0;
    for i in 0..( word.len() - 1 )
    {
        if keyboard_layout.get(&chars[i]).unwrap().contains(chars[i+1])
        {
            difficulty += 1;
        }
    }
    difficulty
}
