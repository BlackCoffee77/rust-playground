pub fn run() {
	let greeting = "Hi";

	//normal strings are immutable, for more general strings use
	let mut better_greeting = String::from("Hi");

	better_greeting.push_str(" o/");
	println!("{} Length: {}", better_greeting, better_greeting.len());

	// Assertion
	assert_eq!(2, greeting.len());
}
