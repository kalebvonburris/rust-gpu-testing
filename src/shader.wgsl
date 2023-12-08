@group(0)
@binding(0)
var<storage, read_write> nums: array<u32>; // this is used as both input and output for convenience

@group(0)
@binding(1)
var<uniform> chunk_size: u32;

const MAX : u32 = 100000u;

fn fib(n: u32) -> u32{
    var second_num: u32 = 0u;
    var first_num: u32 = 1u;
    var current_num: u32 = 1u;

    for (var i = 0u; i <= n; i++) {
        current_num = (second_num + first_num) % MAX;
        second_num = first_num;
        first_num = current_num;
    }
    return current_num;
}

@compute
@workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x * chunk_size;
    // Break out if we exceed the number of needed iteratons
    if idx > arrayLength(&nums) { return; }
    for (var i = 0u; i < chunk_size && idx + i < arrayLength(&nums); i++) {
        nums[idx + i] = fib(nums[idx + i]);
    }
}