pub fn run() {
	let name = "HIII";
	let mut age = 37;

	println!("{} {}", name, age);

	age = 38;

	println!("{} {}", name, age);

	// constants
	const ID: i32 = 100;
	println!("ID: {}", ID);

	//multiples at once
	let (name, age) = ("Zach", 17);

	println!("{} {}", name, age)
}