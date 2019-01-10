use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use memory;
use std::collections::HashMap;

pub fn file_to_codes(path: &str, memory: &mut memory::Memory) -> Vec<u64> {
    let mut file = File::open(path).unwrap();
    let mut codes = Vec::new();

    let instruction_re = Regex::new(r"[\s\t]+?([A-Z]+)$").unwrap();
    let instruction_with_arg_re = Regex::new(r"[\s\t]+?([A-Z]+) ([A-Za-z0-9]+)").unwrap();
    let data_declaration = Regex::new(r"([0-9]+) DAT ([0-9]+)").unwrap();
    let label_declaration = Regex::new(r"([A-Za-z]+):").unwrap();
    let call_declaration = Regex::new(r"CALL ([A-Z]+)").unwrap();
    let ret = Regex::new(r"[\s\t]+?RET").unwrap();
    let mut labels: HashMap<String, u64> = HashMap::new();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut ln = 0;
    let mut call_positions: Vec<(&str, u64)> = Vec::new();
    let mut data_decs: Vec<(u64, u64)> = Vec::new();

    let mut call_loc = 0;

    for line in contents.split("\n") {
        let mut valid = false;

        let ret_match = ret.captures(line);

        let w = data_declaration.captures(line);
        if !w.is_none() {
            let mat = w.unwrap();
            let address = mat.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let data = mat.get(2).unwrap().as_str().parse::<u64>().unwrap();

            data_decs.push((address, data));

            ln -= 1;

            continue;
        }

        if !ret_match.is_none() {
            codes.push(900 + call_loc + 1);
        }

        let label_captures = label_declaration.captures(line);
        if !label_captures.is_none() {
            labels.insert(
                String::from(label_captures.unwrap().get(1).unwrap().as_str()),
                ln,
            );
            continue;
        }

        let call_captures = call_declaration.captures(line);
        if !call_captures.is_none() {
            call_loc = ln;
            let lbl = call_captures.unwrap().get(1).unwrap().as_str();
            call_positions.push((lbl, ln));
            codes.push(0);
            ln += 1;
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
                "JMP" => 900,
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

        if valid {
            ln += 1;
        }

        if !valid {
            if line == "" {
                continue;
            }
            println!("Invalid line: {:?}", line);
        }
    }

    for (label, index) in call_positions {
        codes[index as usize] = 900 + *labels.get(label).unwrap();
    }

    for (index, data) in data_decs {
        memory.items[index as usize] = data;
    }
    (codes)
}

pub fn ops_to_bytes(memory: &memory::Memory, bin: &str) {
    let mut f = File::create(bin).unwrap();

    let mut codes: Vec<u8> = Vec::new();

    for code in &memory.items {
        let code_string = format!("{:03}", code);
        let co = code_string.as_str().chars();
        for c in co {
            codes.push(c.to_digit(10).unwrap() as u8);
        }
    }
    f.write(codes.as_slice()).expect("Could not write file");
}

pub fn bytes_to_ops(bin: &str) -> Vec<u64> {
    let mut f = File::open(bin).expect("No instruction file found");

    let mut ops: Vec<u64> = Vec::new();

    let mut buf: &mut [u8] = &mut [0; 300];

    f.read(&mut buf).expect("Could not read file");

    let chnk = buf.chunks_exact(3);

    for c in chnk {
        ops.push(format!("{}{}{}", c[0], c[1], c[2]).parse::<u64>().unwrap());
    }

    ops
}
