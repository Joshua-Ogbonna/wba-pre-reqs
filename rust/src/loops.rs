pub fn run() {
    let mut count = 0;

    // loop {
    //     count += 1;
    //     println!("{}", count);

    //     if count >= 20 {
    //         break;
    //     }
    // }

    // While loop
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("{}", "FizzBuzz")
    //     } else if count % 3 == 0 {
    //         println!("{}", "Fizz")
    //     } else if count % 5 == 0 {
    //         println!("{}", "Buzz")
    //     } else {
    //         println!("{}", count)
    //     }

    //     // Increment counter
    //     count += 1;
    // }

    // For loop
    for count in 0..10 {
        if count % 15 == 0 {
            println!("{}", "FizzBuzz")
        } else if count % 3 == 0 {
            println!("{}", "Fizz")
        } else if count % 5 == 0 {
            println!("{}", "Buzz")
        } else {
            println!("{}", count)
        }
    }
}

// Loops iterate until a certain condition is met