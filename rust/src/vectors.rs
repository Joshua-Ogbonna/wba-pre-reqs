pub fn run() {
    let mut numbers: Vec<i32> = vec![10, 20, 30, 20];

    // Re-assign value
    numbers[2] = 10;
    numbers.push(30);

    println!("{:?}", numbers);

    // Get single Vector value
    println!("{}", numbers[0]);

    // Slice Vector
    let slice = &numbers[0..2];

    // Get Vector length
    println!("Vector length {}", numbers.len());
    println!("Vector stack allocated {}", std::mem::size_of_val(&numbers));
    println!("Slice {:?}", slice);
    
    // Loop through vector
    for x in numbers.iter() {
        println!("{}", x)
    }
    // Mutate on loop
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers)

}