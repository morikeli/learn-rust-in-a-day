// lesson 14 - shadowing

fn main() {
    let mut x = 10;

    {
        let x = 15;
    }    

    // in shadowing you can change the data type of a variable
    let x = "X is a string";
    println!("x is {}", x);

    let x = true;
    println!("x is {}", x);
}
