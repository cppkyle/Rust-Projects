use std::io;

fn main() {
    println!("Price before tax:");
    let mut price_string = String::new();
    io::stdin()
        .read_line(&mut price_string)
        .expect("Failed to read input");

    let price = price_string.trim().parse::<f64>().expect("That's not a number");

    // Calculate end
    let min_tip = price * 0.15;
    let max_tip = price * 0.20;
    println!("You can pay {} to {} dollars. Press enter to continue", min_tip, max_tip);

    let mut kill = String::new();
    io::stdin()
        .read_line(&mut kill)
        .expect("Failed to read input");
}