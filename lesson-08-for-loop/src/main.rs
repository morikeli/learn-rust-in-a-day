// lesson 08 - for loop
fn main() {
    for i in 1..11 {    // .. is an expression that generates a range. In this case 1 to 10
        println!("The number is {i}")
    }

    let numbers = 30..51;    // range 30 to 50
    

    for i in numbers {
        println!("The number is {}", i);
    }

    // for loop to iterate vectors
    let animals = vec!["Rabbit", "Dog", "Cat"];

    for animal in animals.iter() {
        println!("The animal name is {animal}")
    }

    // using enumerate()
    for (index, animal) in animals.iter().enumerate() {
        println!("Index {index} | The animal name is {animal}")
    }
    
}
