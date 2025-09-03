pub fn compare_results(result1: u32, result2: u32) {
    if result1 > result2 {
        println!("{} is greater than {}", result1, result2);
    } else if result1 < result2 {
        println!("{} is less than {}", result1, result2);
    } else {
        println!("{} is equal to {}", result1, result2);
    }
}
