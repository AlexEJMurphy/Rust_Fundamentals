mod functions;
mod guessing_game;
use functions::another_function;
use guessing_game::guessing_game;
fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // let y = 6;

    // println!("initial main scope of y: {y}");

    // let y = y + 1;

    // {
    //     let y = y * 2;
    //     println!("Inner scope y value: {y}");
    // }
    // //let y = y;
    // println!("main scope y value: {y}");
    
    // let spaces = " ";
    // let spaces = spaces.len();
    // println!("There is {spaces} spaces");

    // let spaces = "   ";
    // let spaces = spaces.len();

    // println!("There is {spaces} spaces");

    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("{guess}");
    // let guess: i32 = guess.reverse_bits().try_into().unwrap();
    // println!("{guess}");
    // let guess: u32 = guess.reverse_bits().try_into().unwrap();
    // println!("{guess}");

    guessing_game();

    another_function(5);

    let mut a = [0];

    let b = a;




}
