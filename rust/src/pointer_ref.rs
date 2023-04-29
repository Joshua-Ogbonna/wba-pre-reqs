pub fn run() {
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    // Vector pointers
    let vec1 = vec![1, 2, 3];

    let _v2 = &vec1;

    println!("{:?}", (&vec1, _v2));
}
