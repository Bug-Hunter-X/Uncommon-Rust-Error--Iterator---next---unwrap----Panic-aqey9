fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    // This will panic if the vector is empty
    let first = iter.next().unwrap();
    println!("First element: {}", first);

    // This is fine
    if let Some(second) = iter.next() {
        println!("Second element: {}", second);
    }
}