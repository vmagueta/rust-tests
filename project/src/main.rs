mod hello;
mod collection;
mod comparison;
mod while_function;
mod looping_for_fun;
mod learn_match;

const SECOND_IN_HOUR: u32 = 60;
const MINUTE_IN_HOUR: u32 = 60;
const HOUR_IN_SECONDS: u32 = SECOND_IN_HOUR * MINUTE_IN_HOUR;

fn main() {
    hello::hello();

    let total = 30;
    let total_in_seconds = total * HOUR_IN_SECONDS;
    println!("Total trabalhado em segundos foi {}", total_in_seconds);

    collection::second();

    comparison::compare_results(total_in_seconds, 30);

    while_function::while_function();

    looping_for_fun::counting_for_fun();

    learn_match::learn_math();
}
