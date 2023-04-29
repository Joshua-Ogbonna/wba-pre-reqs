// mod print;
// mod vars;
// mod types;
// mod string;
// mod tuples;
// mod arrays;
// mod vectors;
// mod conditionals;
// mod loops;
// mod functions;
// mod pointer_ref;
// mod structs;
// mod enums;
mod cli;

fn main() {

    // Funny thing. Coming from JS angle, I don't always give attention to semi-color; buh it's compulsory down 😆
    // println!("Hello, world!");
    cli::run();

    // Basic formatting in Rust
    // println!("{} is from {}", "Josh", "Owerri");

    // Positional Arguments. Best understanding for not repeating yourself. 
    // println!("{0} is from {1} and {0} likes to {2}", "Joshua", "Owerri", "Pray");

    // Named arguments
    // println!("{name} likes to {activity}", name="Joshua", activity="Code");

    // Placeholder traits - Will need to research into this
    // println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 20);

    // Maths Expressions
    // println!("10 + 10 = {}", 10+ 10)
}
