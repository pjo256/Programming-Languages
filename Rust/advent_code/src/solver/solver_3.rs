use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::collections::HashSet;
use std::cmp::min;

fn traverse<F>(map: String, num_santas: i32, get_next: F) -> usize
	where F : Fn(char, &mut (i32, i32)) -> (i32, i32) {
	let mut pos: (i32, i32) = (0, 0);
	let mut other_pos: (i32, i32) = (0, 0);
	let mut directions = map.chars();
	let mut houses_visted = HashSet::new();
	let mut skip = false;

	let mut chars = map.chars();

	for c in chars {
		if (!skip) {
			pos = get_next(c, &mut pos);
			println!("Santa's move {:?}", c);
			houses_visted.insert(pos);
		} else {
			other_pos = get_next(c, &mut other_pos);
			println!("Robo-Santa's move {:?}", c);
			houses_visted.insert(other_pos);
		}

		if (num_santas > 1) {
			skip = !skip;
		}
    }

    houses_visted.len()
}

fn main() {
    let path = Path::new("inputs/input3.txt");
    let display_name = path.display();

    let mut file = match File::open(&path) {
    	Err(reason) => panic!("Couldn't open {}: {}", display_name, 
    												  Error::description(&reason)),
    	Ok(file) => file
    };

	let mut buffer = String::new();

	match file.read_to_string(&mut buffer) {
		Err(reason) => panic!(reason),
		Ok(a) => println!("{}", a)
	}

    println!("{}", traverse(buffer, 2, |c, pos: &mut (i32, i32)| {
    	match c {
    		'>' => pos.0 += 1,
    		'<' => pos.0 -= 1,
    		'^' => pos.1 += 1,
    		'v' => pos.1 -= 1,
    		_ => println!("no mapping for {}", c)
    	}

    	*pos
    }));
} 