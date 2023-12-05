use std::io::{self, Read};

use tasks::task1;
use tasks::task2;

fn main() {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .expect("there's no input");

    let task1_result = task1::solve(&content);
    println!("Solve task 1: {task1_result}");

    let task2_result = task2::solve(&content);
    println!("Solve task 2: {task2_result}");
}
