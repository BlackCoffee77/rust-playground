pub fn run() {
	// default i32
	let x = 1;

	//default f64
	let y = 2.718;

	//explicit

	let z: i64 = 1111111111;

	println!("{}", std::i32::MAX);
	println!("{}", std::i64::MAX);

	let bool_test = true;

	println!("{:?}", (x, y, z, bool_test))
}
