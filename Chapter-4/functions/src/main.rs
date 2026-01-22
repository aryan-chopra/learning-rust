fn main() {
    let num = five();
    print_number(num);
}

fn print_number(num: i32) {
    println!("Num passed is {num}");
}

fn five() -> i32 {
    5
}
