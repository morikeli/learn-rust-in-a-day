// lesson 05 - if ... else statements
fn main() {
    let n = 20;     // change value of n to validate all conditions in the "if ... else if ... else" code block below.


    // NOTE: Unlike in Javascript or C or C++, ts not a must to use parantheses in if statements.
    if n < 30 {
        print!("{} is less than 30", n);
    } else if n == 30 {
        println!("{} is equal to 30", n);
    } else {
        println!("{} is greater than 30", n);   // println! prints a new empty line after the the printed statement. Change "println!" to "print!" to see the difference.
    }
}
