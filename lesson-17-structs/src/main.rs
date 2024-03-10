// lesson 17 - structs
// Structs group similar groups of information into one data type.

struct Color {
    red: u8,    // u8 supports values 0 - 255. In this case suitable for RGB
    green: u8,
    blue: u8,
}

fn main() {
    let mut bg = Color {red: 255, green: 70, blue: 15};

    // To change the value of, let's say, blue, add "mut" keyword in line 11 and write the code statement shown below. 
    bg.blue = 45;

    println!("{}, {}, {}", bg.red, bg.green, bg.blue)
}
