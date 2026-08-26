//make sure you import this line
use std::io;

fn main() {
//Grabs and saves input from the user
    let mut distance = String::new();
    println!("How many miles are you traveling?:");
    io::stdin()
        .read_line(&mut distance)
        .expect("Failed to read line");
//type converts the input from a string to a number
    let distance: i32 = distance
        .trim()
        .parse()
        .expect("Input is not a valid integer");

    let mut efficiency = String::new();
    println!("How many miles per gallon?:");
    io::stdin()
        .read_line(&mut efficiency)
        .expect("Failed to read line");
    let efficiency: i32 = efficiency
        .trim()
        .parse()
        .expect("Input is not a valid integer");

    let mut ppg = String::new();
    println!("What is the price per gallon?:");
    io::stdin()
        .read_line(&mut ppg)
        .expect("Failed to read line");
    let ppg: f64 = ppg.trim().parse().expect("Input is not a valid float");

//prints out the saved inputs
    println!("");
    println!("Distance (miles): {distance}");
    println!("Efficiency (mpg): {efficiency}");
    println!("Price per gallon: {ppg}");

//math to calculate and print out your needed variables
    let gallons_needed = distance / efficiency;
    println!("Gallons needed: {gallons_needed}");
    let gallons_needed: f64 = gallons_needed.into();

    let total_cost = gallons_needed * ppg;
    println!("Total cost: ${total_cost}");
}
