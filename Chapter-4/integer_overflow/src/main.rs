fn main() {
    let guess: u8 = 255;
    
    // Simply wraps the number around the minimum possible value
    let res = guess.wrapping_add(2);
    println!("Wrapping add");
    println!("{res}");
    println!("-------------------------------");
    
    // Returns None value if overflow occurs (Returns Optional<u8>)
    let res = guess.checked_add(1);
    println!("Checked add");
    println!("{:?}", res);
    println!("-------------------------------");
    
    // Returns wrapper answer along with a bool flag specifying if the value overflowed
    let res = guess.overflowing_add(3);
    println!("Overflowing add");
    println!("{:#?}", res);
    println!("-------------------------------");

    // Doesn't let the result overflow the minimum/maximum and saturates it
    let res = guess.saturating_add(10);
    println!("Saturating add");
    println!("{res}");
}
