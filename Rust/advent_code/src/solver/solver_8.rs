extern crate unicode_segmentation;


use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn get_code_len(word: &str) -> usize {
	let split = word.split_word_bounds().collect::<Vec<&str>>();
	//println!("{:?}", split);
	let mut len = 2;

	let escaped_quote = '\"';
	let escaped_backslash = '\\';
	let codepoint = 'x';


	for bound in split {

		let chars: Vec<char> = bound.chars().collect();

		let mut codepoint_start = false;
		let start = chars.get(0).unwrap();

		if (start.is_alphabetic()) {
			if (*start == codepoint && codepoint_start) {
				len += bound.len() - 3;
				codepoint_start = false;
			} else {
				len += bound.len();
			}
		} else {
			//println!("{:?}", chars.get(0));

			let escaped_unicode = *start == escaped_backslash;

			if (escaped_unicode || *start == escaped_quote) {
				codepoint_start = false;
				len += 2;
			} else {
				len += 4;
				codepoint_start = true;
			}

			//let escaped_unicode = chars.get(0).unwrap().escaped_unicode().collect::<Vec<char>>();

			//println!("{:?}", chars.get(0).unwrap().escape_unicode().collect::<Vec<char>>());
		}

		iter.next();
	}

	len
}

fn solver() {

	let literal_chars: i32 = 0;
	let apparent_chars: i32 = 0;
	let quote_offset: i32 = 2;

	let path = Path::new("inputs/input8.txt");
    let display_name = path.display();

    let mut file = match File::open(&path) {
    	Err(reason) => panic!("Couldn't open {}: {}", display_name, 
    												  Error::description(&reason)),
    	Ok(file) => file
    };

    let mut s = String::new();
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
    	let string = line.unwrap();
    	let num_bytes = string.bytes().collect::<Vec<u8>>().len();
    	let w = string.unicode_words().collect::<Vec<&str>>();
    	//let s = "\\\x54rs";
    	let g = UnicodeSegmentation::graphemes(&*string, true).collect::<Vec<&str>>();

    	println!("{:?}", g);
    	println!("{}", num_bytes);
    	println!("{}", get_code_len(&string));

    	total += get_code_len(&string) - num_bytes;


    	//println!("{:?}", w);
    	//println!("bytes {}", num_bytes);
    	//println!("read {:?}", get_readable_string_len(&string));


    }

    let s = "cjex\\tpxaapzf";
    println!("{}", total);

    let ss = "";

    println!("{}", s.len());
    println!("{}", get_code_len(s));

    let g = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();

    println!("{:?}", g);
    //println!("{}", charss.len_utf8());
    




  
}
