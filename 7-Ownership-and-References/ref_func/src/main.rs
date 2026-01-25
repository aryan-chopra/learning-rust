fn main() {
    let s1 = String::from("Hello");
    
    let l = calculate_str_len(&s1);
    
    println!("Size of \"{s1}\" is {l}");
}

fn calculate_str_len(s: &String) -> usize {
    s.len()
}
