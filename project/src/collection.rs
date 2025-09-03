pub const COLLECTION_OF_NUMBERS: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

pub fn second() {
    println!("Starting");
    for i in COLLECTION_OF_NUMBERS.iter() {
        println!("Number: {}", i);
    }
}
