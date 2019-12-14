extern crate clap;
use clap::{Arg, App};
use std::fs;

fn main() {
    let matches = App::new("Intcode Bruteforcer")
        .version("1.0")
        .author("Sven Ewers")
        .about("Intcode Part 2")
        .arg(Arg::with_name("input")
            .required(true)
            .short("i")
            .takes_value(true)
            .help("Path to the input file of the puzzle"))
        .arg(Arg::with_name("required-output")
            .required(true)
            .short("r")
            .takes_value(true)
            .help("Path to the input file of the puzzle"))
        .get_matches();

    let input = fs::read_to_string(
        matches.value_of("input").expect("An input file path is required")
    ).expect("The input file could not be read");
    let required: i32 = matches.value_of("required-output").expect("A required output is required").parse().expect("Required Output needs to be an integer");

    brute_force(parse_input(input), required);
}

fn brute_force(base_input: Vec<i32>, required: i32) {
    for noun in 0..99 {
        for verb in 0..99 {
            print!("Testing noun {} verb {}", noun, verb);
            let mut input = prepare_input(noun, verb, &base_input);
            let result = solve(&mut input);

            if result == required {
                print!(" ✅");
                println!();
                println!("Final result: {}", calculate_result(noun, verb));
                return;
            } else {
                print!(" ❌");
            }
            println!();
        }
    }
}

fn solve(ints: &mut Vec<i32>) -> i32 {
    let mut pos: i32 = 0;
    let mut cont = true;

    while cont {
        // Get values & operation
        let opcode = ints[(pos as usize)];
        let input1 = ints[(ints[((pos+1) as usize)] as usize)];
        let input2 = ints[(ints[((pos+2) as usize)] as usize)];
        let output_location = ints[((pos+3) as usize)] as usize;

        // Perform operation
        match opcode {
            1 => ints[output_location] = input1 + input2,
            2 => ints[output_location] = input1 * input2,

            _ => {
                cont = false;
            }
        }

        pos += 4;
        if pos+3 > (ints.len() as i32) {
            cont = false;
        }

    }

    ints[0]
}

fn parse_input(input: String) -> Vec<i32> {
    let mut ints: Vec<i32> = Vec::new();

    for part in input.split(',') {
        let int: i32 = part.parse().expect("Input file is invalid");
        ints.push(int)
    }

    ints
}

fn prepare_input(noun: i32, verb: i32, input: &Vec<i32>) -> Vec<i32> {
    let mut result = input.to_vec();
    result[1] = noun;
    result[2] = verb;

    result
}

fn calculate_result(noun: i32, verb: i32) -> i32 {
    100 * noun + verb
}