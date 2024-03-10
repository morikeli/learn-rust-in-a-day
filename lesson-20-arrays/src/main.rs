// lesson 20 - arrays

fn main() {
    // let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // specifying data types of items in the array - its not required. Rust does this automatically in the background.
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];   // numbers: [signed data type (i32); len of the array]


    // accessing items in the array
    // numbers[2]    // 3
    // numbers[5]    // 6

    for n in numbers.iter() {
        println!("{}", n);
    }

    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }

}
