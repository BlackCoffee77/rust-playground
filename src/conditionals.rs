pub fn run() {
    let num = 305;

    if num > 20 {
        println!("hi");
    }

    else {
        println!("bye");
    }

    //shorthand
    let num2 = if num > 30 {"yay"} else {"nay"};

    println!("{}", num2);
}