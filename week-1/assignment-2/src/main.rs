use std::io;

fn main() {
    println!("Cafe Discount Calculator");
    println!("Enter your bill amount:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");


    let bill: f64 = input.trim().parse().expect("Please enter a number");

    let discount;
    if bill > 10000.0 {
        discount = 15.0;
    } else if bill > 5000.0 {
        discount = 10.0;
    } else {
        discount = 0.0;
    }

    let final_bill = bill - (bill * discount / 100.0);

    println!("Original Bill: {}", bill);
    println!("Discount Applied: {}%", discount);
    println!("Final Bill: {}", final_bill);
}
