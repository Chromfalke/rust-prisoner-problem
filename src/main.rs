use std::io::Write;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use std::process::exit;

const STRATEGIES: [&str; 2] = ["random", "loop"];
const ITERATIONS: i32 = 10000;
const PRISONERS: i32 = 100;

fn random_strategy(prisoner: i32, boxes: &Vec<i32>) -> bool {
    let start_number: usize = prisoner.try_into().unwrap();
    let mut closed: Vec<i32> = boxes.to_vec();
    for _i in 0..50 {
        let mut rng = thread_rng();
        let random_box = rng.gen_range(0..closed.len());
        let next_number: usize = boxes[random_box].try_into().unwrap();
        closed.remove(random_box);
        if next_number == start_number {
            return true;
        }
    }
    return false;
}

fn loop_strategy(prisoner: i32, boxes: &Vec<i32>) -> bool {
    let start_number: usize = prisoner.try_into().unwrap();
    let mut next_number: usize = prisoner.try_into().unwrap();
    for _i in 0..50 {
        next_number = boxes[next_number].try_into().unwrap();
        if next_number == start_number {
            return true;
        }
    }
    return false;
}

fn run_iteration(strategy: usize) -> bool {
    let mut boxes: Vec<i32> = (0..PRISONERS).collect();
    boxes.shuffle(&mut thread_rng());
    for prisoner in 0..PRISONERS {
        let result: bool = match strategy {
            0 => random_strategy(prisoner, &boxes),
            1 => loop_strategy(prisoner, &boxes),
            _ => exit(1),
        };
        if !result {
            return false;
        }
    }
    return true;
}

fn main() {
    println!("Select Strategy");
    for i in 0..STRATEGIES.len() {
        println!("{0}) {1}", i, STRATEGIES[i]);
    }
    let mut user_selection = String::new();
    print!("[0]>");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut user_selection)
    	.ok()
        .expect("Failed to read line");
    user_selection = user_selection.replace("\n", "");
    let selected_strategy = match user_selection.parse::<usize>() {
        Ok(num) if num <= STRATEGIES.len() => num,
        _ => 0,
    };
    println!("User selected {}", &STRATEGIES[selected_strategy]);

    let mut success_count = 0;
    let mut failure_count = 0;
    for _i in 0..ITERATIONS {
        let result: bool = run_iteration(selected_strategy);
        if result {
            success_count += 1;
        }
        else {
            failure_count += 1;
        }
    }

    println!("Successes: {}", success_count);
    println!("Failures: {}", failure_count);
}