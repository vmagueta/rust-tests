pub fn learn_math() {
    let number = 2;

    match number {
        1 => println!("You got one!"),
        2 => println!("You got two!"),
        3..=10 => println!("Between three and ten"),
        _ => println!("Something else"),
    }
}
