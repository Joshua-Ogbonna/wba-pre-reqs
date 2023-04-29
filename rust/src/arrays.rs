
pub fn run() {
    let mut numbers: [i32; 3] = [10, 20, 30];

    // Re-assign value
    numbers[2] = 10;

    println!("{:?}", numbers);

    // Get single array value
    println!("{}", numbers[0]);

    // Slice array
    let slice = &numbers[0..2];

    // Get array length
    println!("Array length {}", numbers.len());
    println!("Array stack allocated {}", std::mem::size_of_val(&numbers));
    println!("Slice {:?}", slice)
}

// Arrays have fixed length; of same data types