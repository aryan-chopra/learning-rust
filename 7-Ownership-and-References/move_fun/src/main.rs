fn main() {
    let s1 = String::from("Hello");
    
    let (s1, len) = calc_str_len(s1);
    
    println!("Size of \"{s1}\" is {len}");
}

fn calc_str_len(s: String) -> (String, usize) {
    let length = s.len();
    
    (s, length)
}
