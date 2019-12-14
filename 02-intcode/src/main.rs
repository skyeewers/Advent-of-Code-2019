extern crate clap;
use clap::{Arg, App};
use std::fs;

fn main() {
    let matches = App::new("Intcode Interpreter")
        .version("1.0")
        .author("Sven Ewers")
        .about("Interpret Intcode Part 1")
        .arg(Arg::with_name("input")
            .required(true)
            .index(1)
            .help("Path to the input file of the puzzle"))
        .get_matches();

    let input = fs::read_to_string(
        matches.value_of("input").expect("An input file path is required")
    ).expect("The input file could not be read");

    solve(input);
}

fn solve(input: String) {
    let mut ints = parse_input(input);
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
                println!("Encountered end");
                println!("Complete content:");
                debug_output(&ints);
                println!("Value at pos 0 is: {}", ints[0].to_string());
                cont = false;
            }
        }

        pos += 4;
        if pos+3 > (ints.len() as i32) {
            cont = false;
            println!("Complete content:");
            debug_output(&ints);
        }

    }
}

fn parse_input(input: String) -> Vec<i32> {
    let mut ints: Vec<i32> = Vec::new();

    for part in input.split(',') {
        let int: i32 = part.parse().expect("Input file is invalid");
        ints.push(int)
    }

    ints
}

fn debug_output(output: &Vec<i32>) {
    for row in output.chunks(4) {
        for integer in row {
            print!("{} ", integer);
        }
        // println!(" ");
    }
    println!(" ");
}