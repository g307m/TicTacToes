use super::con;
use super::vars;
pub fn grid() -> () {
    con::clear();
    con::goto(1,1);
    let mut fullr: String = "".to_string();
    let mut emptr: String = "".to_string();
    for i in 0..7 {
        fullr = fullr+"#";
    }
    for i in 0..4 {
        emptr = emptr+"# ";
    }
    println!("{}",fullr);// #######
    println!("{}",emptr);// # # # #
    println!("{}",fullr);// #######
    println!("{}",emptr);// # # # #
    println!("{}",fullr);// #######
    println!("{}",emptr);// # # # #
    println!("{}",fullr);// #######
}
pub fn play(player:bool,x:i8,y:i8) -> () {
    if(player){
        super::board[1] = 'x';
    }else{

    }
}
