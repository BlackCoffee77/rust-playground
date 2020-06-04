pub fn run() {
	let my_tuple: (&str, &str, i8) = ("hi", "tired", 81);
	println!("{} {} {}", my_tuple.0, my_tuple.1, my_tuple.2);
}