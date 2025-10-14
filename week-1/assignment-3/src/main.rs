use std::io;

fn main() {
    println!("EKECDC Smart Meter Billing System");
    println!("Enter your electricity usage (in kWh):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let units: f64 = input.trim().parse().expect("Please enter a number");

    let rate;
    if units > 200.0 {
        rate = 30.0;
    } else if units > 100.0 {
        rate = 25.0;
    } else {
        rate = 20.0;
    }

    let total_bill = units * rate;

    println!("-----------------------------");
    println!("Units Used: {} kWh", units);
    println!("Rate per Unit: {}", rate);
    println!("Total Bill: {}", total_bill);
    println!("-----------------------------");
}
