// The numbers used here reflect 5/25/2018's numbers

use std::io;

fn main() {
    let cad_to_us = 0.771301;
    let us_to_cad = 1.29651;

    println!("Amount of money to convert:");

    let mut money_string = String::new();
    io::stdin()
        .read_line(&mut money_string)
        .expect("Failed to read input");

    let money = money_string.trim().parse::<f64>().expect("That's not a number");

    println!("1- Canadian Dollar to US Dollar\n2- US Dollar to Canadian Dollar");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to get choice");
    let choice_proper = choice.trim().parse::<u8>().expect("You did not insert a correct choice number!");

    if choice_proper == 1 {
        let converted = money * cad_to_us;
        println!("{} Canadian Dollars are equal to {} US Dollars", money, converted);
    } else if choice_proper == 2 {
        let converted = money * us_to_cad;
        println!("{} US Dollars are equal to {} Canadian Dollars", money, converted);
    } else {
        println!("You did not insert a correct choice number!")
    }

    println!("Press enter to continue");
    let mut kill = String::new();
    io::stdin()
        .read_line(&mut kill)
        .expect("Error");

}
