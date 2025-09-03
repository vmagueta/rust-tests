pub fn while_function() {
    let mut counter = 0;
    while counter < 5 {
        println!("Counter is at {}", counter);
        counter += 1;
    }
    println!("Final counter value is {}", counter);
}
