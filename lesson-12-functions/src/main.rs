// lesson 12 - functions
fn main() {
    print_numbers_to(10);
}

fn print_numbers_to(num: u32) {
    for n in 1..num {
        if is_even(n) {
            println!("{} is even!", n)
        } else {
            println!("{} is odd.", n);
        }
    }
}

// return a value. In this case a boolean
fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}
