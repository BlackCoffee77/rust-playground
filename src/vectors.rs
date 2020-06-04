use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers[2] = 7;
    numbers.push(5);
    numbers.pop();

    println!("{:?}", numbers);
    println!("{}", numbers[0]);

    // size of vector
    println!("How many bytes: {}", mem::size_of_val(&numbers));

    //[1 : 3)
    let slice: &[i32] = &numbers[1..3];
    println!("Slice {:?}", slice);

    for x in numbers.iter() {
        println!("{}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 10;
    }
    println!("{:?}", numbers);
}