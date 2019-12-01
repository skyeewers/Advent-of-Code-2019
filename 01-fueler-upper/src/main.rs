extern crate clap;
use clap::{Arg, App};
use std::fs;

fn main() {
    let matches = App::new("Fueler Upper")
        .version("1.0")
        .author("Sven Ewers")
        .about("Calculate Fuel requirements to get to Santa")
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
 let modules: Vec<&str> = raw_input.split("\n").collect();
 let mut total: f32 = 0.0;

 for module in &modules {
     let parsed: f32 = module.parse().unwrap();
     let divided: f32 = parsed / 3.0;
     let rounded: f32 = divided.floor();
     total += rounded - 2.0;
 }

 println!("The ammount of fuel required is {}", total);
}