fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    if vec.len() > 2 {
        let value = vec[2];
        println!("Value: {}", value);
    } else {
        println!("Index out of bounds");
    }
    //Alternatively using get()
    match vec.get(2) {
        Some(value) => println!("Value: {}", value),
        None => println!("Index out of bounds")
    }
} 