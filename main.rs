use std::io;

fn main() {
    println!("🌡️ Smart Weather Temperature Converter");
    println!("Enter a temperature followed by its unit (C/F)");

    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 2 {
        println!("Invalid input format");
        return;
    }

    let value: f64 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    let unit = parts[1].to_uppercase();

    
    if unit == "C" {
        let fahrenheit = (value * 9.0 / 5.0) + 32.0;
        println!("Temperature: {:.1}°C", value);
        println!("Converted: {:.1}°F", fahrenheit);
    } else if unit == "F" {
        let celsius = (value - 32.0) * 5.0 / 9.0;
        println!("Temperature: {:.1}°F", value);
        println!("Converted: {:.1}°C", celsius);
    } else {
        println!("Unknown unit '{}'. Use 'C' or 'F'.", unit);
    }
}
