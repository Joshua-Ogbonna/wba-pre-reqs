
enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar (m: Movement) {
    match m {
        Movement::Down => println!("Avatar moving down"),
        Movement::Up => println!("Avatar moving up"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right")
    }
}

pub fn run() {
    let avatar1 = Movement::Down;
    let ava2 = Movement::Left;
    let ava3 = Movement::Right;
    let ava4 = Movement::Up;
    
    move_avatar(avatar1)
}