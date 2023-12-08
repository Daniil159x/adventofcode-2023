use std::env;
use std::io::{self, Read};

use tasks::task1;
use tasks::task2;

fn main() {
    let task_num = match env::args().nth(1) {
        Some(arg) => arg.parse().expect("first arg must be task number"),
        None => 1,
    };

    if ![1, 2].contains(&task_num) {
        panic!("only 1 or 2 allowed");
    }

    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .expect("there's no input");

    match task_num {
        1 => {
            let task1_result = task1::solve(&content);
            println!("Solve task 1: {task1_result}");
        }
        2 => {
            let task2_result = task2::solve(&content);
            println!("Solve task 2: {task2_result}");
        }
        _ => unreachable!(),
    }
}
