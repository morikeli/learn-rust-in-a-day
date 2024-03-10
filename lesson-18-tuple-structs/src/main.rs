// lesson 18 - tuple structs

struct Color (u8, u8, u8);  // this matches with the struct Color in lesson 17. Unlike in lesson 17, tuple structs only have data type - not data type and members.


fn main() {
    let red = Color(255, 0, 0);
    
    println!("Red is {}, {}, {}", red.0, red.1, red.2)  // .0, .1, .2 are indices in the tuple struct
}
