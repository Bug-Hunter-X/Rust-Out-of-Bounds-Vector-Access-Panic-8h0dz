fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 10;

    // Use get() to safely access elements, checking for None
    match vec.get(index) {
        Some(element) => println!("Element at index {} is: {}", index, element),
        None => println!("Index {} is out of bounds", index),
    };
}