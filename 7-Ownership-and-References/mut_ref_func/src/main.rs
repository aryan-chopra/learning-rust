fn main() {
    let mut s1 = String::from("Hello");
    
    append(&mut s1, &String::from(", world"));
    
    let refs1 = &s1;
    println!("After mutation: {refs1}");
}

fn append(string_to_append_to: &mut String, string_to_append: &String) {
    string_to_append_to.push_str(string_to_append);
}
