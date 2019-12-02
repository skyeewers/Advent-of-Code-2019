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
    let fuel = fuel_requirement(raw_input);
    
    println!("The total ammount of fuel required is {}", fuel);
}

fn fuel_requirement(raw_input: String) -> f32 {
 let modules: Vec<&str> = raw_input.split('\n').collect();
 let mut total: f32 = 0.0;

 for module in &modules {
     let mut module_total = 0.0;
     let parsed: f32 = module.parse().unwrap();
     let divided: f32 = parsed / 3.0;
     let rounded: f32 = divided.floor();
     module_total += rounded - 2.0;
     module_total = add_tirany_requirement(module_total);
     total += module_total;
 }

 total
}

fn add_tirany_requirement(current_fuel: f32) -> f32 {
   let mut total = current_fuel;
   let mut new_requirement = (current_fuel / 3.0).floor() - 2.0;

   while new_requirement > 0.0 {
       total += new_requirement;
       println!("Added {}", new_requirement);
       new_requirement = (new_requirement / 3.0).floor() - 2.0;
   }

   total
}