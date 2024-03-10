// lesson 11 - tuples

fn main() {
    let tup1 = (20, 30, 40, 50, 60);
    let tup2 = (20, "Rust", 3.4, false);
    let tup3 = (20, 35, 50, 60, (1, 2, 3, 4, 5));
    let tup4 = (45, 6.7, "computer");    // Destructuring
    let (a, b, c) = tup4;

    println!("{}", tup1.2);  // print the value of the tuple at index 2 using "." In this case it will print "40" because its in index 2

    println!("{}", tup2.3);

    println!("{}", (tup3.4).3);  // access items at index 4 in tup3 and item at index 3 in the inner tuple

    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);

}
