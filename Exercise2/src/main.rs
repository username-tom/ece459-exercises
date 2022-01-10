// You should implement the following function
fn fibonacci_number(n: u32) -> u32 {
    let mut f0 = 0u32;
    let mut f1 = 1u32;
    let mut sum = 0u32;
    for _i in 1..n {
        sum = f0 + f1;
        f0 = f1;
        f1 = sum;
    }
    sum
}


fn main() {
    println!("{}", fibonacci_number(10));
}
