fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    // Safe handling of potential empty iterator
    if let Some(first) = iter.next() {
        println!("First element: {}", first);
        if let Some(second) = iter.next() {
            println!("Second element: {}", second);
        }
    } else {
        println!("The vector is empty!");
    }
} 