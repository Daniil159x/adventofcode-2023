use std::io::{self, Read};

use tasks::{cubes::Cubes, task1, task1_structs, task2_structs};
// use tasks::task2;

fn main() {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .expect("there's no input");

    let task1_result = task1::solve(&content, 12, 13, 14);
    println!("Solve task 1: {task1_result}");

    let task1_result = task1_structs::solve(&content, Cubes::new(12, 13, 14));
    println!("Solve task 1 with structs: {task1_result}");

    let task2_result = task2_structs::solve(&content);
    println!("Solve task 2 with structs: {task2_result}");

    // let task2_result = task2::solve(&content);
    // println!("Solve task 2: {task2_result}");
}
