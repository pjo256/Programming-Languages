use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::collections::HashSet;
use std::cmp::min;

/*
fn knapsack(capacity: i32, ingredient_ind: i32, score: &[i32], look_up: &[[i32]]) {
	if capacity == 0 || ingredient_ind == score.len() {
		return 0;
	} else if look_up[ingredient_ind][capacity] != -1 {
		return look_up[ingredient_ind][capacity];
	} else {

		let add_tsp = score[ingredient_ind] + knapsack(capacity - 1, ingredient_ind + 1, score, look_up);
		let no_tsp = knapsack(capacity, ingredient_ind + 1, score, look_up);

		let max_val: i32 = 0;
		if (add_tsp > no_tsp) {
			max_val = add_tsp;
		} else {
			max_val = no_tsp;
		}

		look_up[ingredient_ind][capacity] = max_val
	}
}
*/

fn main() {
    let path = Path::new("solver/inputs/input15.txt");
    let display_name = path.display();

    let mut file = match File::open(&path) {
    	Err(reason) => panic!("Couldn't open {}: {}", display_name, 
    												  Error::description(&reason)),
    	Ok(file) => file
    };

	let reader = BufReader::new(file);

	for line in reader.lines() {
		let mut tokens: Vec<_> = line.unwrap().split(':').collect();
		let ingredient = tokens.get(0).unwrap();
		let stats: Vec<_> = line.to_str().unwrap().split(',').collect();

	}


    
} 