
fn next_sequence(input: &str) -> String {
	let mut chars = input.chars();
	let mut start = chars.next().unwrap();

	let mut output = String::new();

	loop {
		match chars.by_ref().next() {
			None => {
				output.push_str(&("1".to_string() + &*start.to_string()));
				//println!("{:?}", output);
				return output;
			},
			Some(c) => {
				let mut len: i32 = 1;
				let mut next = c;
				//println!("start = {}", start);	
				//println!("{}", c);
				while start == next {
					len += 1;
					//println!("Curr {:?}, next {:?}", start, next);

					match chars.by_ref().next() {
						None => break,
						Some(c) => {
							next = c;
						}
					}
				}

				let s_len = len.to_string();
				let s_c = &*start.to_string();
				//println!("{}, {}", s_len, s_c);
	
				output.push_str(&(s_len + s_c));

				start = next;
			}
		}
	}

}

fn solver() {

	let mut output: String = "".to_string();
	let mut input: &str = "1113122113";
	let mut output = next_sequence(input); //3 1's 1 3 1 1 2 2's 2 1's 1 3
	let mut i = 1;

	while i < 50 {
		output = next_sequence(&*output);
		i += 1;
	}
	
	println!("{}", output.len());
}
