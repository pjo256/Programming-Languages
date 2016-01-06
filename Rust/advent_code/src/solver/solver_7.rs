use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::collections::HashMap;

fn lshift(a: i32, b: i32) -> i32 {
	a << b
}

fn rshift(a: i32, b: i32) -> i32 {
	a >> b
}

fn and(a: i32, b: i32) -> i32 {
	a & b
}

fn or(a: i32, b: i32) -> i32 {
	a | b
}

fn not(a: i32) -> i32 {
	!a
}

fn solver() {
	let BINARY_OP_COUNT = 5;
	let ASSIGNMENT_COUNT = 3;

    let path = Path::new("inputs/input7.txt");
    let display_name = path.display();

    let mut file = match File::open(&path) {
    	Err(reason) => panic!("Couldn't open {}: {}", display_name, 
    												  Error::description(&reason)),
    	Ok(file) => file
    };

    let mut s = String::new();
    let reader = BufReader::new(file);

    let mut operators = HashMap::new();
    operators.insert("RSHIFT", rshift as fn(i32, i32) -> i32);
    operators.insert("LSHIFT", lshift as fn(i32, i32) -> i32);
    //operators.insert("NOT", not as fn(i32) -> i32);
    operators.insert("AND", and as fn(i32, i32) -> i32);
    operators.insert("OR", or as fn(i32, i32) -> i32);

    let mut variable_bindings = HashMap::new();

    while (!variable_bindings.contains('a')) {
    	for line in reader.lines() {
			let s = line.unwrap();
			let mut tokens: Vec<_> = s.split(' ').collect();
			let len = {tokens.len()};
	
			let mut token_iter = tokens.iter_mut();
	
			let var = token_iter.next().unwrap();
	
			match len {
				ASSIGNMENT_COUNT => {

					match var.parse::<i32>() {
						Ok(a) => {

						}, 
						Err(_) => {
							
						}
					}
					let target = token_iter.next().next();


					variable_bindings.insert(target, var);
	
				
				},
				BINARY_OP_COUNT - 1 => {
	
				},
				BINARY_OP_COUNT => {
	
				}
			}
		}
    }
} 