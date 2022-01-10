// You should implement the following function:

fn sum_of_multiples(mut number: i32, multiple1: i32, multiple2: i32) -> i32 {
    let mut sum = 0i32;
    /*for i in 1..number {
        if i % multiple1 == 0 {
            sum += i;
        } else if i % multiple2 == 0 {
            sum += i;
        }
    }
    The above way prompts the compiler to ask to remove mut in front of number. */
    loop {
        number -= 1;
        if (number % multiple1 == 0) | (number % multiple2 == 0) {
            sum += number;
        }
        if number == 0 {
            break;
        }
    }
    sum
}

fn main() {
    println!("{}", sum_of_multiples(1000, 5, 3));
}
