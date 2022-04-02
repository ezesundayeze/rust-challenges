use std::fs;

fn main(){
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("Contents is {}", contents);

    for content in contents.lines(){
        println!("{}", content)
    }
}