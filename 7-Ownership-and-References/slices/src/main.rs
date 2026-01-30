fn main() {
    let s = String::from("Hello, world");
    
    let first_word = first_word(&s);
    
    println!("First word: {first_word}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (index, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return &s[0..index];
        }
    }
    
    &s[..]
}
