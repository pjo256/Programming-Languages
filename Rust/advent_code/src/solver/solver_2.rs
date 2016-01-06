use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::collections::HashMap;
use std::cmp::min;

fn surface_area_slack(l: u32, w: u32, h: u32) -> u32 {
	let lw = l * w;
	let wh = w * h;
	let hl = h * l;
	2 * lw + 2 * wh + 2 * hl + min(min(lw, wh), hl)
}

fn feet(l: u32, w: u32, h: u32) -> u32 {
	let lw = 2 * (l + w);
	let wh = 2 * (w + h);
	let hl = 2 * (h + l);
	let v = l * w * h;
	min(min(lw, wh), hl) + v
}

fn solver() {
    let path = Path::new("inputs/input2.txt");
    let display_name = path.display();

    let mut file = match File::open(&path) {
    	Err(reason) => panic!("Couldn't open {}: {}", display_name, 
    												  Error::description(&reason)),
    	Ok(file) => file
    };

    let reader = BufReader::new(file);

    let mut total_sa = 0;
    let mut total_feet = 0;

    for line in reader.lines() {
    	let s = line.unwrap();
		let mut dimensions: Vec<&str> = s.split('x').collect();
		let len = dimensions.get(0).unwrap().parse::<u32>().ok().unwrap();
		let width = dimensions.get(1).unwrap().parse::<u32>().ok().unwrap();
		let height = dimensions.get(2).unwrap().parse::<u32>().ok().unwrap();

		total_sa += surface_area_slack(len, width, height);
		total_feet += feet(len, width, height);

    }
    println!("{}", total_sa);
    println!("{}", total_feet);
    	
} 