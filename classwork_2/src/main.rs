use std::io;

fn main() {
    println!("Enter your total bill amount (N): ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    let bill: f64 = input.trim().parse().expect("Please enter a valid number");

    let discount_rate = if bill > 10_000.0{
        0.15
    } else if bill > 5_000.0 {
        0.10
    } else {
        0.0
    };

    let discount_amount = bill * discount_rate;
    let final_bill = bill - discount_amount;

    println!("Original Bill: N{:.2}", bill);
    println!("Discount Applied: {}%", discount_rate * 100.0);
    println!("Final Bill after Discount: N{:.2}", final_bill);
}
