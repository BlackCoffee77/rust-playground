pub fn run() {
    greeting("hi", "bob");
    let sum = add(1, 3);
    println!("{}", sum);

    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("{}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

// functions return non-semicolon line
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}