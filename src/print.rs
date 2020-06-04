pub fn run() {
    // Print to console
    println!("Number: {} {}", 1, 2);

    println!("ree {0} {1} {0}", "a", "B");

    println!("{name} likes {thing}", name="AAA", thing="BBB");

    println!("Binary {:b} Hex {:x} Octal {:o}", 10, 10, 10);

    // All the other ones are normal, but this is wonky
    // This is like a kinda catchall for debugging
    println!("{:?}", (12, true, "hi"))
}