use std::io;

fn main() {
    println!("Enter your energy consumption in kwh: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let usage: f64 = input.trim().parse().expect("Please enter a valid number");

    let rate_per_unit = if usage > 200.0 {
        30.0
    } else if usage > 100.0 {
        25.0
    } else {
        20.0
    };

    let bill = usage * rate_per_unit;

    println!("\n========== E K E D C  Smart  Meter ==========");
    println!("Energy Consumption: {:.2} kWh", usage);
    println!("Rate per Unit: ₦{:.2}", rate_per_unit);
    println!("---------------------------------------------");
    println!("Total Electricity Bill: ₦{:.2}", bill);
    println!("=============================================");

}
