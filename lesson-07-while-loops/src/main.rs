// lesson 07 - while loop
// In this lesson, write a program to print numbers from 1 - 50 
// that are multiples of 5.

fn main() {
    let mut n = 1;

    while n <= 50 {
        // if n is a multiple of 5
        if n % 5 == 0 {
            println!("n is {n}")
        }

        // increment n
        n += 1;
    }
    
}
