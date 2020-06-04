pub fn run() {
	let mut count = 0;

	// while true
	loop {
		count += 1;
		println!("{}", count);

		if count == 20 {
			break;
		}
	}

	//simple fizzbuzz
	count = 0;
	while count <= 1000 {
		let mut output = String::from("");
		if count % 3 == 0 {
			output.push_str("fizz");
		}

		if count % 5 == 0 {
			output.push_str("buzz");
		}
		if output.eq("") {
			println!("{}", count);
		} else {
			println!("{}", output);
		}
		count += 1;
	}

	// implementation with for
	for count in 0..1000 {
		let mut output = String::from("");
		if count % 3 == 0 {
			output.push_str("fizz");
		}

		if count % 5 == 0 {
			output.push_str("buzz");
		}
		if output.eq("") {
			println!("{}", count);
		} else {
			println!("{}", output);
		}
	}
}
