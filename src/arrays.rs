use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    numbers[2] = 7;

    println!("{:?}", numbers);
    println!("{}", numbers[0]);

    // size of arrays
    println!("How many bytes: {}", mem::size_of_val(&numbers));

    //[1 : 3)
    let slice: &[i32] = &numbers[1..3];
    println!("Slice {:?}", slice)
}