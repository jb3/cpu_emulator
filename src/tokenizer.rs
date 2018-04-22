use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use memory;

pub fn file_to_codes(path: &str, memory: &mut memory::Memory) -> Vec<u64> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let z = contents.split("\n");

    let mut codes = Vec::new();

    let instruction_re = Regex::new(r"([A-Z]+)$").unwrap();
    let instruction_with_arg_re = Regex::new(r"([A-Z]+) ([A-Za-z0-9]+)").unwrap();
    let data_declaration = Regex::new(r"([0-9]+) DAT ([0-9]+)").unwrap();

    for line in z {
        let mut valid = false;

        let w = data_declaration.captures(line);
        if !w.is_none() {
            let mat = w.unwrap();
            let address = mat.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let data = mat.get(2).unwrap().as_str().parse::<u64>().unwrap();

            memory.items[(address - 1) as usize] = data;

            continue;
        }

        let x = instruction_re.captures(line);

        if !x.is_none() {
            let opcode = match x.unwrap().get(1).unwrap().as_str() {
                "IN" => 500,
                "OUT" => 600,
                "HLT" => 700,
                _ => 700,
            };
            valid = true;
            codes.push(opcode);
        }

        let y = instruction_with_arg_re.captures(line);

        if !y.is_none() {
            let mat = y.unwrap();
            let op = match mat.get(1).unwrap().as_str() {
                "LDA" => 100,
                "STA" => 200,
                "ADD" => 300,
                "SUB" => 400,
                "SET" => 800,
                _ => 700,
            };

            let addr = mat.get(2).unwrap().as_str().parse::<u64>().unwrap();

            if addr > 99 {
                panic!("Requested address is greater than 99")
            }

            let finish = op + addr;
            valid = true;
            codes.push(finish);
        }

        if !valid {
            if line == "" {
                continue;
            }
            println!("Invalid line: {:?}", line);
        }
    }

    codes
}

pub fn ops_to_bytes(memory: memory::Memory) {
    let mut f = File::create("out.bin").unwrap();

    let mut codes: Vec<u8> = Vec::new();

    for code in memory.items {
        let code_string = format!("{:03}", code);
        let co = code_string.as_str().chars();
        for c in co {
            codes.push(c.to_digit(10).unwrap() as u8);
        }
    }
    f.write(codes.as_slice()).expect("Could not write file");
}

pub fn bytes_to_ops() -> Vec<u64> {
    let mut f = File::open("out.bin").expect("No instruction file found");

    let mut ops: Vec<u64> = Vec::new();

    let mut buf: &mut [u8] = &mut [0; 100];

    f.read(&mut buf).expect("Could not read file");

    let chnk = buf.exact_chunks(3);

    for c in chnk {
        ops.push(format!("{}{}{}", c[0], c[1], c[2]).parse::<u64>().unwrap());
    }

    ops
}
