// lesson 10 - constants
// constants are declared in the global scope
// constant names should be in uppercase and use snake casing to indicate spaces

const MAXIMUM_NUMBER: u8 = 20;


fn main() {
    for n in 1..MAXIMUM_NUMBER {
        println!("The number is {n}")
    }    
}
