// lesson 16 - references
fn main() {
    let mut x = 10;
    // let xr = &x;    // '&' means you want to get a variable next to the ampersand

    {
        let name = &mut x;
        *name += 1;
    }


    // For example
    println!("x is {}", x);     // change 'x' to 'xr' and see the results. The reult will be the same.

    // println!("x is {}", name);
}
