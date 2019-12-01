extern crate clap;
use clap::{Arg, App};
use std::fs;

fn main() {
    let matches = App::new("Fueler Upper Part 2")
        .version("2.0")
        .author("Sven Ewers")
        .about("Calculate Fuel requirements to get to Santa! Now handling the rocket equation")
        .arg(Arg::with_name("input")
            .required(true)
            .index(1)
            .help("Path to the input file of the puzzle"))
        .get_matches();

    let input = fs::read_to_string(
        matches.value_of("input").expect("An input file path is required")
    ).expect("The input file could not be read");

    solve(input)
}

fn solve(raw_input: String) {
    let moduleFuel = module_requirement(raw_input);
    let tiranyFuel = rocketEquationFuel(moduleFuel);
    let result = moduleFuel + tiranyFuel;

    println!("Dry parts require {} of fuel", moduleFuel);
    println!("An additional {} of fuel is required to satisfy the tirany of the rocket equation", tiranyFuel);
    println!("The total ammount of fuel required is {}", result);
}

fn module_requirement(raw_input: String) -> f32 {
 let modules: Vec<&str> = raw_input.split("\n").collect();
 let mut total: f32 = 0.0;

 for module in &modules {
     let parsed: f32 = module.parse().unwrap();
     let divided: f32 = parsed / 3.0;
     let rounded: f32 = divided.floor();
     total += rounded - 2.0;
 }

 total
}

fn rocketEquationFuel(moduleFuel: f32) -> f32 {
    let mut nextStepRequired: bool = true;
    let mut fuel = (moduleFuel / 3.0) - 2.0;
    let mut lastAdded = fuel;

    while ((lastAdded / 3.0).floor() - 2.0) > 0.0 {
        lastAdded = (lastAdded / 3.0).floor() - 2.0;
        fuel += lastAdded;
    }

    fuel
}