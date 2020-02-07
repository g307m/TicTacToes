pub fn clear()->() {
    println!("{}[2J", 27 as char);
}
pub fn goto(xpos:i8,ypos:i8)-> () {
    print!("{}[{};{}H", 27 as char,xpos,ypos);
}
