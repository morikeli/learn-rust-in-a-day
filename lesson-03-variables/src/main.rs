// let's learn how to use variables in Rust
fn main() {
    let mut x = 45;

    println!("The value of x is {x}");

    // By default variables in Rust are immutable, i.e. cannot be changed. To make variables mutable, use the 'mut' keyword. 
    // See line 3. 'mut' keyword is used to make the variable mutable.
    x = 60;

    println!("The value of x is {}", x);
}
