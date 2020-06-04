pub fn run() {
	let arr1 = [1, 2, 3];
	let arr2 = arr1;

	println!("{:?}", (arr1, arr2));

	// doesn't work cuz vector is non-primitive, need to use pointer ref
	// let vec1 = vec![1, 2, 3];
	// let vec2 = vec1;

	let vec1 = vec![1, 2, 3];
	let vec2 = &vec1;



}