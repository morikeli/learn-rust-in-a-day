// lesson 09 - enum types

enum Direction {
    Up,
    Down,
    Right,
    Left
}

fn main() {
    let player_direction: Direction = Direction::Down;    // :: is used to access a variant. In this case the variant is "Up"

    // match works like a switch statement
    match player_direction {
        Direction::Up => println!("Player is moving forward."),
        Direction::Right => println!("Player is moving the right direction."),
        Direction::Down => println!("Player is moving backward."),
        Direction::Left => println!("Player is moving to the left direction."),

    }
}
