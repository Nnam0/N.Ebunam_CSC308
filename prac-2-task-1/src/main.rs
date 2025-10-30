fn main() {
    let sentence = String::from("Hello world from Rust");
    let last = last_word(&sentence);
    println!("The last word is: {}", last);
}

fn last_word(s: &str) -> &str {
    let words: Vec<&str> = s.split_whitespace().collect();
    match words.last() {
        Some(&word) => word,
        None => "",
    }
}
