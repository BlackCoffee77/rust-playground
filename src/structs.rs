// custom data types
// kinda like classes

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct ColorTuple(u8, u8, u8);

//struct with functions
struct Person {
    first: String,
    last: String,
    health: i8,
}

impl Person {
    //constructor
    fn new(first: &str, last: &str, health: i8) -> Person {
        Person{
            first: first.to_string(),
            last: last.to_string(),
            health: health
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first, self.last)
    }

    fn get_health(&self) -> i8 {
        self.health
    }

    fn take_damage(&mut self, dam: i8) {
        self.health -= dam;
    }
}

pub fn run() {
    let mut c = Color {red: 255, green: 0, blue: 0};
    c.red = 200;

    println!("{} {} {}", c.red, c.green, c.blue);

    let mut c_tuple = ColorTuple(255, 0, 0);

    println!("{} {} {}", c_tuple.0, c_tuple.1, c_tuple.2);

    let mut p = Person::new("john", "doe", 100);
    p.take_damage(5);
    println!("{}", p.get_health());

}