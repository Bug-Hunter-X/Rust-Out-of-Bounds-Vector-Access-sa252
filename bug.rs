fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let value = vec[2]; // This will panic because index 2 is out of bounds
    println!("Value: {}", value);
}