#[macro_use]
extern crate timeit;

use rust_gpu_testing::*;

use rayon::prelude::*;

#[tokio::main]
async fn main() {
    // Calculate fin(500,000) 500,000 times
    let numbers: Vec<_> = vec![500_000; 500_000];

    let steps_gpu = execute_gpu(numbers.clone()).await.unwrap();
    let steps_cpu = execute_cpu(numbers.clone());

    steps_gpu.par_iter().enumerate().for_each(|(index, gpu_ans)|{
        if *gpu_ans != steps_cpu[index] {
            println!(
                "GPU Answer: {} != CPU Answer: {}",
                gpu_ans, steps_cpu[index]
            );
            panic!("Your code is wrong!");
        }
    });

    println!("Answers recieved were identical between the GPU and CPU.");

    println!("Benching CPU...");

    let cpu_time = timeit_loops!(5, {
        execute_cpu(numbers.clone());
    });

    println!("Benching GPU...");

    let gpu_time = timeit_loops!(5, {
        execute_gpu(numbers.clone()).await.unwrap();
    });

    println!(
        "CPU time: {}, GPU time: {}, ratio: {}",
        cpu_time,
        gpu_time,
        cpu_time / gpu_time
    );
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
    current_num
}
