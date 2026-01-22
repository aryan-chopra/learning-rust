use std::io;

fn main() {
    println!("Enter the temperature in Fahrenheit");
    
    let mut temperature_in_fahrenheit = String::new();
    
    io::stdin()
        .read_line(&mut temperature_in_fahrenheit)
        .expect("Failed to read input");
    
    let temperature_in_fahrenheit: f64 = match temperature_in_fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Temperature must be a valid number");
            return
        }
    };

    let temperature_in_celsius: f64 = (temperature_in_fahrenheit - 32f64) * (5f64 / 9f64);
    
    println!("Temperature in celsius is {temperature_in_celsius}");
}
