#[macro_use]
extern crate timeit;

use rust_gpu_testing::*;

use rayon::prelude::*;

#[tokio::main]
async fn main() {
    let numbers: Vec<_> = (0..50_000).collect();

    println!("Benching GPU...");

    timeit!({
        execute_gpu(numbers.clone()).await.unwrap();
    });

    println!("Benching CPU...");

    timeit!({
        execute_cpu(numbers.clone());
    });

    let steps_gpu = execute_gpu(numbers.clone()).await.unwrap();
    let steps_cpu = execute_cpu(numbers.clone());

    for (index, gpu_ans) in steps_gpu.iter().enumerate() {
        if *gpu_ans != steps_cpu[index] {
            println!(
                "GPU Answer: {} != CPU Answer: {}",
                gpu_ans, steps_cpu[index]
            );
            panic!("Your code is wrong!");
        }
    }

    println!("Answers recieved were identical.");
}

const MAX: u32 = 100000;

fn execute_cpu(numbers: Vec<u32>) -> Vec<u32> {
    return numbers.par_iter().map(|num| fib(*num)).collect();
}

fn fib(n: u32) -> u32 {
    let mut second_num: u32 = 0;
    let mut first_num: u32 = 1;
    let mut current_num: u32 = 1;

    for _ in 0..(n + 1) {
        current_num = (second_num + first_num) % MAX;
        second_num = first_num;
        first_num = current_num;
    }
    return current_num;
}
