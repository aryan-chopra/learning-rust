use std::io;

fn main() {
    println!("Please enter the position of the number which you want from the fibonacci sequence");
    
    let mut n = String::new();
    
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");

    let n: i64 = n.trim().parse().expect("Please enter a valid integer");
    
    if n < 3 {
        println!("Nth fibonacci number is {}", n - 1);
        return;
    }
    
    let mut a: i64 = 0;
    let mut b: i64 = 1;
    
    for sequence in 3..n + 1{
        let c = a + b;
        
        if sequence == n {
            println!("Nth fibonacci number is {c}");
            break;
        }
        
        a = b;
        b = c;
    }
}
