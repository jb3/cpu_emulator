#![feature(exact_chunks)]

extern crate regex;

use regex::Regex;

use std::io;
use std::env::args;
use std::process::exit;

mod instructions;
use instructions::*;

mod memory;
mod tokenizer;

fn main() {
    let args = args().collect::<Vec<String>>();
    let mut mem = memory::Memory::new();
    if args.contains(&String::from("--compile")) {
        let fi = args.get(
            args.binary_search(&String::from("--compile"))
                .expect("You need to pass in a file if you specify --compile") + 1,
        ).unwrap();
        let codes = tokenizer::file_to_codes(&fi, &mut mem);
        for (i, v) in codes.into_iter().enumerate() {
            mem.items[i] = v;
        }

        tokenizer::ops_to_bytes(mem);
        println!("Success! Your code has been saved.");
        exit(0);
    }

    if args.contains(&String::from("--run")) {
        let codes = tokenizer::bytes_to_ops();
        println!("Running compiled binary...");

        for (i, v) in codes.into_iter().enumerate() {
            mem.items[i] = v;
        }

        execute(&mut mem);
        exit(0);
    }
    println!(
        "Joseph's CPU Emulator\n\n\tUsage: {} [--option] [file]\n\n\t\t--compile [file]        Compile the given file\n\t\t--run                   Run the compiled bin",
        args[0]
    );
}

fn execute(memory: &mut memory::Memory) {
    let mut accumulator = 0;

    for i in memory.items.clone() {
        if i == 0 {
            continue;
        }
        let x = parse(i);
        match x.kind {
            InstructionType::Load => {
                println!(
                    "Loading value at address {} into accumulator",
                    x.address - 1
                );
                accumulator = memory.items[(x.address as usize) - 1];
            }
            InstructionType::Store => {
                println!(
                    "Storing value from accumulator in memory address {}",
                    x.address - 1
                );
                memory.items[(x.address as usize) - 1] = accumulator;
            }
            InstructionType::Input => {
                let mut inp = String::new();

                println!("INPUT: ");

                io::stdin()
                    .read_line(&mut inp)
                    .expect("Reading input failed");

                let number = inp.trim().parse::<u64>().unwrap();
                accumulator = number;
            }
            InstructionType::Output => println!("{}", accumulator),
            InstructionType::Add => {
                println!(
                    "Adding value at memory address {} to accumulator",
                    x.address - 1
                );
                accumulator += memory.items[(x.address as usize) - 1];
            }
            InstructionType::Subtract => {
                println!(
                    "Subtracting value at memory address {} from accumulator",
                    x.address - 1
                );
                accumulator -= memory.items[(x.address as usize) - 1];
            }
            InstructionType::Halt => {
                break;
            }
        }
    }

    println!("Accumulator:       {}", accumulator);
    println!("Memory table: ");
    println!("╭─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────╮");
    let mem_slice = memory.items.as_slice();
    let chunks = mem_slice.chunks(10);
    for (i, chunk) in chunks.enumerate() {
        print!("│");
        let v: Vec<String> = chunk
            .to_vec()
            .iter()
            .map(|&x| format!(" {:03} │", x))
            .collect::<Vec<String>>();

        for x in &v {
            print!("{}", x);
        }
        print!("\n");

        if i != 9 {
            println!("├─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┤");
        }
    }
    println!("╰─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────╯");
}

fn parse(code: u64) -> Instruction {
    let code_string = format!("{}", code);
    let code_string = code_string.as_str();

    if code_string.len() > 3 {
        panic!(
            "Received {} digit instruction when expecting one of 3 digits.",
            code_string.len()
        );
    }

    let re = Regex::new(r"([0-9])([0-9]{2})").unwrap();

    let caps = re.captures(code_string).unwrap();

    let op = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
    let addr = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
    let instruction_type = match op {
        1 => InstructionType::Load,
        2 => InstructionType::Store,
        3 => InstructionType::Add,
        4 => InstructionType::Subtract,
        5 => InstructionType::Input,
        6 => InstructionType::Output,
        7 => InstructionType::Halt,
        _ => panic!("Unexpected opcode: {}", op),
    };

    Instruction {
        kind: instruction_type,
        address: addr,
    }
}
