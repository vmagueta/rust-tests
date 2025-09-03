pub fn counting_for_fun() {
    let mut count = 0;
    { 
        loop {   
            println!("Count is {}", count);
            count += 1;

            if count == 8 {
                break;
            }
        }
    }
}
