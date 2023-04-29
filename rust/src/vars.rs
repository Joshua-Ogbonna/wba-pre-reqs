pub fn run() {
    //  Rust variables are immutable, meaning it can't be assigned
    let name = "Josh";
    let mut age = 32;

    age = 40;

    println!("My name is {}", name);
    println!("My age is {}", age);

    // Constant variables
    const ID: i32 = 001;
    const DOB: &str = "23/08/1996";

    println!("ID: {}", ID);
    println!("DOB: {}", DOB);

    // Multiple variables assignments
    let (greeting, say_yes) = ("Hello World", "Yeahhhhhh");
    println!("{} {}", greeting, say_yes);
}