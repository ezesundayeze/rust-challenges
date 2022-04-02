use std::env;

fn main(){
    // check the length of a command line argument
    if env::args().len() <2{
        println!("You must pass a parameter");
        return;
    }
    //Loop through the items in a commandline argument
    for (index, argument) in env::args().enumerate(){
        println!("{}, {}", index, argument);
    }

    // Get the value of the argument in different positions, which is really the first argument
    let mut argument = env::args();
    println!("Argument: {} {}", argument.nth(0).unwrap(), argument.nth(2).unwrap())
}
