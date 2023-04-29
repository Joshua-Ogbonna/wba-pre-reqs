// Traditional struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple struct
// struct  Color(u8, u8, u8);
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name (&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };
    // let mut c = Color(255, 0, 0);

    let mut person = Person::new("John", "Doe");

    // println!("Colors: {} {} {}", c.red, c.green, c.blue)
    // println!("Colors: {} {} {}", c.0, c.1, c.2);

    println!("Person {} {}", person.first_name, person.last_name);
    println!("{}", person.full_name());
}

// Structs is used to handle custom data types
