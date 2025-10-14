use std::io;

fn main() {
    println!("🌤️ Smart Weather Temperature Converter");
    println!("Enter the temperature value (e.g. 36.5):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let temp: f64 = input.trim().parse().expect("Please enter a valid number");

    println!("Is this in (C)elsius or (F)ahrenheit? Enter C or F:");

    let mut scale = String::new();
    io::stdin().read_line(&mut scale).expect("Failed to read input");
    let scale = scale.trim().to_uppercase();

    if scale == "C" {
        let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
        println!("Temperature: {:.1}°C", temp);
        println!("Converted: {:.1}°F", fahrenheit);
    } else if scale == "F" {
        let celsius = (temp - 32.0) * 5.0 / 9.0;
        println!("Temperature: {:.1}°F", temp);
        println!("Converted: {:.1}°C", celsius);
    } else {
        println!("Invalid unit entered. Please type 'C' or 'F'.");
    }
}
