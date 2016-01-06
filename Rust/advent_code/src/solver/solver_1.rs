use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::collections::HashMap;

fn solver() {
    let path = Path::new("inputs/input1.txt");
    let display_name = path.display();

    let mut file = match File::open(&path) {
    	Err(reason) => panic!("Couldn't open {}: {}", display_name, 
    												  Error::description(&reason)),
    	Ok(file) => file
    };

	let mut buffer = String::new();

	match file.read_to_string(&mut buffer) {
		Err(reason) => panic!("wtf"),
		Ok(a) => println!("{}", a)
	}

    let mut chars = buffer.chars();
    let mut floor = 0;
    let mut pos = 1;

    for c in chars {
    	if (c == '(') {
    		floor += 1;
    	} else {
    		floor -= 1;
    	}

    	if (floor == -1) {
    		break;
    	}

    	pos += 1;
    }

    println!("{}", floor);
    println!("basement at pos {}", pos);

    
} 