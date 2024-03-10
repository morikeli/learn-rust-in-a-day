// lesson 06 - infinite loop using "loop" keyword
fn main() {
    let mut n = 0;

    loop {
        n += 1;

        // using 'continue' keyword
        // 7 will not be printed when the program is compiled
        if n == 7 {
            continue;
        }

        // using an if statement to stop the infinite loop using break
        if n > 20 {
            break;
        }

        println!("The value of n is {}", n)
    }
}
