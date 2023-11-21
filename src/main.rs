use rust_gpu_testing::run;

fn main() {
    pollster::block_on(run());
}