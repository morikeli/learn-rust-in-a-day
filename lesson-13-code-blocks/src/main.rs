// lesson 13 - code blocks
// A code block is a piece of code in {}

fn main() {
    let x = 10;

    {
        let y = 5;

        println!("x: {} | y: {}", x, y);
    }

    // the print statement below will generate an error, because it can't find.
    // println!("x: {} | y: {}", x, y);

}
