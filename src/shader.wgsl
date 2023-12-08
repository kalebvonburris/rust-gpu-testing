@group(0)
@binding(0)
var<storage, read_write> v_indices: array<u32>; // this is used as both input and output for convenience

const MAX : u32 = 100000u;

// The Collatz Conjecture states that for any integer n:
// If n is even, n = n/2
// If n is odd, n = 3n+1
// And repeat this process for each new n, you will always eventually reach 1.
// Though the conjecture has not been proven, no counterexample has ever been found.
// This function returns how many times this recurrence needs to be applied to reach 1.
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
    v_indices[global_id.x] = fib(v_indices[global_id.x]);
}